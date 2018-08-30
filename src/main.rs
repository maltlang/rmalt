mod value;
mod core;

fn main() {
    let em = "hello";
    let emm = || {
        || { em.to_string() }
    };
    println!("{}", emm()());
}
