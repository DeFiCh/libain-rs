use heck::{ToPascalCase, ToSnekCase};
use proc_macro2::{Span, TokenStream};
use prost_build::{Config, Service, ServiceGenerator};
use quote::quote;
use syn::{Fields, GenericArgument, Ident, Item, ItemStruct, PathArguments, Type};

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{DirEntry, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{env, fs, io};

fn visit_files(dir: &Path, f: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_files(&path, f)?;
            } else {
                f(&entry);
            }
        }
    }
    Ok(())
}

// Type-level attributes that need to be added for proto structs (like serde)
const TYPE_ATTRS: &'static [(&'static str, &'static str)] = &[(
    ".types",
    "#[derive(Serialize)] #[serde(rename_all=\"camelCase\")]",
)];

// Field-level attributes that need to be added for proto structs (like serde)
const FIELD_ATTRS: &'static [(&'static str, &'static str)] = &[
    // (".blockchain.version_hex", "#[serde(rename = \"versionHex\")]"),
];

// Custom generator to collect RPC call signatures
struct WrappedGenerator {
    methods: Rc<RefCell<HashMap<String, Vec<RPC>>>>,
    inner: Box<dyn ServiceGenerator>,
}

#[derive(Debug)]
struct RPC {
    name: String,
    input_ty: String,
    output_ty: String,
}

impl ServiceGenerator for WrappedGenerator {
    fn generate(&mut self, service: Service, buf: &mut String) {
        for method in &service.methods {
            let mut ref_map = self.methods.borrow_mut();
            let vec = ref_map.entry(service.name.clone()).or_insert(vec![]);
            vec.push(RPC {
                name: method.proto_name.clone(),
                input_ty: method.input_proto_type.clone(),
                output_ty: method.output_proto_type.clone(),
            });
        }
        self.inner.generate(service, buf);
    }

    fn finalize(&mut self, buf: &mut String) {
        self.inner.finalize(buf);
    }
}

fn generate_from_protobuf(dir: &Path, out_dir: &Path) -> HashMap<String, Vec<RPC>> {
    let methods = Rc::new(RefCell::new(HashMap::new()));
    let gen = WrappedGenerator {
        methods: methods.clone(),
        inner: tonic_build::configure()
            .build_client(false)
            .service_generator(),
    };

    let mut protos = vec![];
    visit_files(&dir, &mut |entry: &DirEntry| {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".proto") {
            println!("cargo:rerun-if-changed={}", path.display());
            protos.push(path);
        }
    })
    .expect("visiting files");

    {
        // There's no way to compile protos using custom generator in tonic,
        // so we're left with creating a prost config and using that for codegen.
        let mut config = Config::new();
        config.out_dir(out_dir);
        config.service_generator(Box::new(gen));
        for (path, attrs) in TYPE_ATTRS {
            config.type_attribute(path, attrs);
        }
        for (path, attrs) in FIELD_ATTRS {
            config.field_attribute(path, attrs);
        }
        config
            .compile_protos(&protos, &[dir])
            .expect("compiling protobuf");
    } // drop it so we release rc count

    Rc::try_unwrap(methods).unwrap().into_inner()
}

fn modify_codegen(
    methods: HashMap<String, Vec<RPC>>,
    types_path: &Path,
    rpc_path: &Path,
) -> TokenStream {
    let mut contents = String::new();
    File::open(types_path)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let parsed_file = syn::parse_file(&contents).unwrap();
    let (ffi_tt, impl_tt, rpc_tt) = apply_substitutions(parsed_file, methods);

    // Append additional RPC impls next to proto-generated RPC impls
    contents.clear();
    File::open(rpc_path)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    let mut codegen = String::new();
    codegen.push_str("\n#[cxx::bridge]\nmod ffi {\n\n");
    codegen.push_str(&ffi_tt.to_string());
    codegen.push_str("\n}\n");
    codegen.push_str(&impl_tt.to_string());
    for (server_mod, tt) in rpc_tt {
        let (server, service, svc_mod, svc_trait) = (
            Ident::new(
                &format!("{}Server", server_mod.to_pascal_case()),
                Span::call_site(),
            ),
            Ident::new(
                &format!("{}Service", server_mod.to_pascal_case()),
                Span::call_site(),
            ),
            Ident::new(
                &format!("{}Server", server_mod).to_snek_case(),
                Span::call_site(),
            ),
            Ident::new(&server_mod.to_pascal_case(), Span::call_site()),
        );
        codegen.push_str(
            &quote!(
                pub struct #service;

                impl #service {
                    #[inline]
                    pub fn service() -> #svc_mod::#server<#service> {
                        #svc_mod::#server::new(#service)
                    }
                }

                #[tonic::async_trait]
                impl #svc_mod::#svc_trait for #service {
                    #tt
                }
            )
            .to_string(),
        );
    }
    contents.push_str(&codegen);
    File::create(rpc_path)
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();

    codegen.parse().unwrap() // given to cxx codegen
}

fn generate_cxx_glue(tt: TokenStream, target_dir: &Path) {
    let codegen = cxx_gen::generate_header_and_cc(tt, &cxx_gen::Opt::default()).unwrap();
    File::create(target_dir.join("libain.hpp"))
        .unwrap()
        .write_all(&codegen.header)
        .unwrap();
    File::create(target_dir.join("libain.cpp"))
        .unwrap()
        .write_all(&codegen.implementation)
        .unwrap();
}

fn apply_substitutions(
    file: syn::File,
    methods: HashMap<String, Vec<RPC>>,
) -> (TokenStream, TokenStream, HashMap<String, TokenStream>) {
    let mut map = HashMap::new();
    let mut gen = quote!();
    // Replace prost-specific fields with defaults
    for item in file.items {
        let mut s = match item {
            Item::Struct(s) => s,
            _ => continue,
        };

        map.insert(s.ident.to_string(), s.clone());
        let empty_struct: ItemStruct = syn::parse2(quote! {
            #[derive(Default)]
            struct S;
        })
        .unwrap();

        s.attrs = empty_struct.attrs;
        let fields = match &mut s.fields {
            Fields::Named(ref mut f) => f,
            _ => panic!("unsupported struct"),
        };

        for field in &mut fields.named {
            field.attrs.clear(); // clear attributes
            fix_type(&mut field.ty);
        }

        gen.extend(quote! {
            #s
        });
    }

    // FIXME: We don't have to regenerate if the struct only has scalar types
    // (in which case it'll have the same schema in both FFI and protobuf)

    let mut impls = quote!();
    let mut calls = HashMap::new();
    for s in map.values() {
        let mut copy_block_rs = quote!();
        let mut copy_block_ffi = quote!();
        let fields = match &s.fields {
            Fields::Named(ref f) => f,
            _ => unreachable!(),
        };

        for field in &fields.named {
            let name = &field.ident;
            let ty = &field.ty;
            let t = quote!(#ty).to_string().replace(" ", "");
            let (into_rs, into_ffi) = if t.contains("::core::option::") {
                (
                    quote!(Some(other.#name.into())),
                    quote!(other.#name.map(Into::into).unwrap_or_default()),
                )
            } else if t.contains("::alloc::vec::") {
                (
                    quote!(other.#name.into_iter().map(Into::into).collect()),
                    quote!(other.#name.into_iter().map(Into::into).collect()),
                )
            } else {
                (quote!(other.#name.into()), quote!(other.#name.into()))
            };

            copy_block_rs.extend(quote!(
                #name: #into_rs,
            ));
            copy_block_ffi.extend(quote!(
                #name: #into_ffi,
            ));
        }

        let name = &s.ident;
        impls.extend(quote!(
            impl From<ffi::#name> for super::types::#name {
                fn from(other: ffi::#name) -> Self {
                    super::types::#name {
                        #copy_block_rs
                    }
                }
            }

            impl From<super::types::#name> for ffi::#name {
                fn from(other: super::types::#name) -> Self {
                    ffi::#name {
                        #copy_block_ffi
                    }
                }
            }
        ));
    }

    let mut rpc = quote!();
    for (mod_name, mod_methods) in methods {
        let server_mod = calls.entry(mod_name).or_insert(quote!());
        for method in mod_methods {
            let (name, name_rs, ivar, ity, oty) = (
                Ident::new(&method.name, Span::call_site()),
                Ident::new(&method.name.to_snek_case(), Span::call_site()),
                Ident::new(
                    &method.input_ty.split(".").last().unwrap().to_snek_case(),
                    Span::call_site(),
                ),
                Ident::new(
                    &method.input_ty.split(".").last().unwrap(),
                    Span::call_site(),
                ),
                Ident::new(
                    &method.output_ty.split(".").last().unwrap(),
                    Span::call_site(),
                ),
            );
            let (input_rs, input_ffi, into_ffi, call_ffi) =
                if method.input_ty == ".google.protobuf.Empty" {
                    (
                        quote!(&self, _request: tonic::Request<()>),
                        quote!(),
                        quote!(),
                        quote!(),
                    )
                } else {
                    (
                        quote!(&self, request: tonic::Request<super::types::#ity>),
                        quote!(#ivar: &mut #ity),
                        quote! { let mut #ivar = request.into_inner().into(); },
                        quote!(&mut #ivar),
                    )
                };
            rpc.extend(quote!(
                fn #name(#input_ffi) -> Result<#oty>;
            ));
            server_mod.extend(quote!(
                async fn #name_rs(#input_rs) -> Result<tonic::Response<super::types::#oty>, tonic::Status> {
                    #into_ffi
                    let result = ffi::#name(#call_ffi).map_err(|e| tonic::Status::unknown(e.to_string()))?;
                    Ok(tonic::Response::new(result.into()))
                }
            ));
        }
    }

    gen.extend(quote!(
        unsafe extern "C++" {
            #rpc
        }
    ));

    (gen, impls, calls)
}

fn fix_type(ty: &mut Type) {
    let t = quote!(#ty).to_string().replace(" ", "");
    if t.contains("::prost::alloc::string::") {
        *ty = syn::parse2(quote!(String)).unwrap();
    }
    if t.contains("::prost::alloc::vec::") {
        let mut inner = get_path_bracketed_ty_simple(&ty);
        fix_type(&mut inner);
        *ty = syn::parse2(quote!(Vec<#inner>)).unwrap();
    }
    if t.contains("::core::option::") {
        *ty = get_path_bracketed_ty_simple(&ty);
    }
}

/// Extracts "T" from std::option::Option<T> for example
fn get_path_bracketed_ty_simple(ty: &Type) -> Type {
    match ty {
        Type::Path(ref p) => {
            let last = p.path.segments.last().unwrap();
            match &last.arguments {
                PathArguments::AngleBracketed(ref a) => match a.args.first().unwrap() {
                    GenericArgument::Type(ref t) => return t.clone(),
                    _ => panic!("unsupported generic type: {}", quote!(#ty)),
                },
                PathArguments::None => return ty.clone(),
                _ => panic!("parenthesis type {} not supported", quote!(#ty)),
            }
        }
        _ => panic!("unsupported type {}", quote!(#ty)),
    }
}

fn main() {
    let mut root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    root.pop();
    let out_dir = env::var("OUT_DIR").unwrap();
    let methods = generate_from_protobuf(&root.join("protobuf"), Path::new(&out_dir));
    let tt = modify_codegen(
        methods,
        &Path::new(&out_dir).join("types.rs"),
        &Path::new(&out_dir).join("rpc.rs"),
    );
    generate_cxx_glue(tt, &root.join("target"));
}
