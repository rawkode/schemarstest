use jsonschema::JSONSchema;

fn main() {
    let file = std::fs::File::open("schema.json").unwrap();
    let reader = std::io::BufReader::new(file);
    let schema = serde_json::from_reader(reader).unwrap();

    let file = std::fs::File::open("data.json").unwrap();
    let reader = std::io::BufReader::new(file);
    let instance = serde_json::from_reader(reader).unwrap();

    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    let result = compiled.validate(&instance);
    assert!(result.is_ok())
}
