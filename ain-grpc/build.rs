use heck::ToSnekCase;
use proc_macro2::{TokenStream, Span};
use prost_build::{Config, Service, ServiceGenerator};
use quote::quote;
use syn::{Fields, Ident, Item, ItemStruct, GenericArgument, PathArguments, Type};

use std::{env, fs, io};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{DirEntry, File};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use std::rc::Rc;

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
const TYPE_ATTRS: &'static [(&'static str, &'static str)] = &[
    (".types", "#[derive(Serialize)] #[serde(rename_all=\"camelCase\")]"),
];

// Field-level attributes that need to be added for proto structs (like serde)
const FIELD_ATTRS: &'static [(&'static str, &'static str)] = &[
    // (".blockchain.version_hex", "#[serde(rename = \"versionHex\")]"),
];

// Custom generator to collect RPC call signatures
struct WrappedGenerator {
    methods: Rc<RefCell<Vec<RPC>>>,
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
            self.methods.borrow_mut().push(RPC {
                name: method.proto_name.clone(),
                input_ty: method.input_proto_type.clone(),
                output_ty: method.output_proto_type.clone(),
            });
        }
        self.inner.generate(service, buf);
    }
}

fn generate_from_protobuf(dir: &Path, out_dir: &Path) -> Vec<RPC> {
    let methods = Rc::new(RefCell::new(Vec::new()));
    let gen = WrappedGenerator {
        methods: methods.clone(),
        inner: tonic_build::configure().service_generator(),
    };

    let mut protos = vec![];
    visit_files(&dir, &mut |entry: &DirEntry| {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".proto") {
            println!("cargo:rerun-if-changed={}", path.display());
            protos.push(path);
        }
    }).expect("visiting files");

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
        config.compile_protos(&protos, &[dir])
              .expect("compiling protobuf");
    }

    Rc::try_unwrap(methods).unwrap().into_inner()
}

fn generate_cxx_glue(methods: Vec<RPC>, path: &Path, target_dir: &Path) {
    let mut contents = String::new();
    let mut fd = File::open(path).unwrap();
    fd.read_to_string(&mut contents).unwrap();

    let parsed_file = syn::parse_file(&contents).unwrap();
    let (ffi_tt, impl_tt) = apply_substitutions(parsed_file, &methods);
    contents.push_str("\n#[cxx::bridge]\nmod ffi {\n\n");
    contents.push_str(&ffi_tt.to_string());
    contents.push_str("\n}\n");
    contents.push_str(&impl_tt.to_string());

    // Append additional structs and impls next to proto-generated structs
    let mut fd = File::create(path).unwrap();
    fd.write_all(contents.as_bytes()).unwrap();

    let tt = contents.parse().unwrap();
    let codegen = cxx_gen::generate_header_and_cc(tt, &cxx_gen::Opt::default()).unwrap();
    File::create(target_dir.join("libain.hpp")).unwrap()
        .write_all(&codegen.header).unwrap();
    File::create(target_dir.join("libain.cpp")).unwrap()
        .write_all(&codegen.implementation).unwrap();
}

fn apply_substitutions(file: syn::File, methods: &[RPC]) -> (TokenStream, TokenStream) {
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
        }).unwrap();

        s.attrs = empty_struct.attrs;
        let fields = match &mut s.fields {
            Fields::Named(ref mut f) => f,
            _ => panic!("unsupported struct"),
        };

        for field in &mut fields.named {
            field.attrs.clear();  // clear attributes
            fix_type(&mut field.ty);
        }

        gen.extend(quote! {
            #s
        });
    }

    // FIXME: We don't have to regenerate if the struct only has scalar types
    // (in which case it'll have the same schema in both FFI and protobuf)

    let mut wrapper = quote!();
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
                (quote!(Some(other.#name.into())), quote!(other.#name.map(Into::into).unwrap_or_default()))
            } else if t.contains("::alloc::vec::") {
                (quote!(other.#name.into_iter().map(Into::into).collect()),
                 quote!(other.#name.into_iter().map(Into::into).collect()))
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
        wrapper.extend(quote!(
            impl From<ffi::#name> for #name {
                fn from(other: ffi::#name) -> Self {
                    #name {
                        #copy_block_rs
                    }
                }
            }

            impl From<#name> for ffi::#name {
                fn from(other: #name) -> Self {
                    ffi::#name {
                        #copy_block_ffi
                    }
                }
            }
        ));
    }

    let mut rpc = quote!();
    for method in methods {
        let (name, name_rs, ivar, ity, oty) = (
            Ident::new(&method.name, Span::call_site()),
            Ident::new(&method.name.to_snek_case(), Span::call_site()),
            Ident::new(&method.input_ty.split(".").last().unwrap().to_snek_case(), Span::call_site()),
            Ident::new(&method.input_ty.split(".").last().unwrap(), Span::call_site()),
            Ident::new(&method.output_ty.split(".").last().unwrap(), Span::call_site()),
        );
        let (input_rs, input_ffi, into_ffi, call_ffi) = if method.input_ty == ".google.protobuf.Empty" {
            (quote!(), quote!(), quote!(), quote!())
        } else {
            (quote!(mut #ivar: #ity), quote!(#ivar: &mut #ity), quote!{ let mut #ivar = #ivar.into(); }, quote!(&mut #ivar))
        };
        rpc.extend(quote!(
            fn #name(#input_ffi) -> Result<#oty>;
        ));
        wrapper.extend(quote!(
            pub fn #name_rs(#input_rs) -> Result<#oty, cxx::Exception> {
                #into_ffi
                let result = ffi::#name(#call_ffi)?;
                Ok(result.into())
            }
        ));
    }

    gen.extend(quote!(
        unsafe extern "C++" {
            #rpc
        }
    ));

    (gen, wrapper)
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
        },
        _ => panic!("unsupported type {}", quote!(#ty)),
    }
}

fn main() {
    let mut root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    root.pop();
    let out_dir = env::var("OUT_DIR").unwrap();
    let methods = generate_from_protobuf(&root.join("protobuf"), Path::new(&out_dir));
    generate_cxx_glue(methods, &Path::new(&out_dir).join("types.rs"), &root.join("target"));
}
