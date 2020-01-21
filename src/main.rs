use inflector::Inflector;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Error: needs 2 arguments");
        std::process::exit(1);
    }
    let inflector_type = &args[1];
    let input = &args[2];
    let output = match inflector_type.as_str() {
        "camel" => input.to_camel_case(),
        "kebab" => input.to_kebab_case(),
        "pascal" => input.to_pascal_case(),
        "sentence" => input.to_sentence_case(),
        "snake" => input.to_snake_case(),
        "screaming" => input.to_screaming_snake_case(),
        "ugly" => input.to_train_case().replace("-", "_"),
        _ => {
            eprintln!("Unknown type: {}", inflector_type);
            std::process::exit(1);
        }
    };
    println!("{}", output)
}
