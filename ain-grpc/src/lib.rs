#[macro_use]
extern crate serde;
extern crate serde_json;

mod codegen;

#[cxx::bridge]
mod server {
    struct Block {
        hash: String,
    }

    extern "Rust" {
        fn show_block();
    }

    unsafe extern "C++" {
        fn fill_block(block: &mut Block) -> Result<()>;
    }
}

fn show_block() {
    let mut block = self::server::Block { hash: String::new() };
    self::server::fill_block(&mut block).unwrap();
    println!("\n{}\n", block.hash);
}
