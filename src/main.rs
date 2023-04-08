use llm_landing_page;
fn main() {
    let result = llm_landing_page::main();

    match result {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}
