pub mod asm;
pub mod vm;

enum Example {
    HelloWorld,
}

fn gather_example() -> Example {
    Example::HelloWorld
}

fn main() {
    let example = gather_example();
    match example {
        Example::HelloWorld => {
            println!("TODO");
        }
    }
}
