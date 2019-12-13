extern crate libucl;

fn main() {
    let parser = libucl::Parser::new();

    let config = match parser.parse_file("examples/test.conf") {
        Ok(conf) => conf,
        Err(err) => panic!("{:?}", err)
    };

    println!("{:?}", config.fetch("lol").and_then(|val| val.as_string()));
    println!("{:?}", config.fetch_path("placki.duze").and_then(|val| val.as_bool()));
    println!("{:?}", config.fetch_path("placki.średnica").and_then(|val| val.as_int()));
    println!("{:?}", config.fetch_path("non.existent.path").and_then(|val| val.as_string()));
}
