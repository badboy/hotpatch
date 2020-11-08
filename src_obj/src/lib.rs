use phf::phf_map;

#[no_mangle]
pub static HOTPATCH_EXPORTS: phf::Map<&'static str, fn() -> i32> = phf_map! {
    "::foo" => foo,
};

pub fn foo() -> i32 {
    println!("Hello from foo");
    1
}