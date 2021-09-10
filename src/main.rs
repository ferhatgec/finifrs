// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

extern crate inifrs;

struct Finif {
    file_data: String,
    __init: inifrs::Inif
}

impl Default for Finif {
    fn default() -> Self {
        Finif {
            file_data: "".to_string(),
            __init     : Default::default()
        }
    }
}

impl Finif {
    fn init(&mut self, filename: String) {
        self.file_data = std::fs::read_to_string(filename).unwrap_or("".to_string());
        self.__init.parse(self.file_data.clone());
    }

    fn get(&mut self, argument: String) -> String {
        let data: Vec<_> = argument.split(':').collect();

        return if data.len() > 1 {
            self.__init.get(data[0].to_string(), data[1].to_string())
        } else { "\"\"".to_string() };
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("finifrs - cli .ini parser");
        std::process::exit(1);
    }

    let mut init = Finif::default();
    init.init(args[1].clone());

    for i in 2..args.len() {
        println!("\x1b[0;97m{}\x1b[0m", init.get(args[i].clone()));
    }
}