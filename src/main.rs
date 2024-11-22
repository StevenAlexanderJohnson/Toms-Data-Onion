use std::io::{Read, Write};

fn get_file_content(file_path: &str) -> String {
    let mut file = std::fs::File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

fn write_file_content(file_path: &str, content: &[u8]) {
    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(content).unwrap();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <layer number>", args[0]);
        std::process::exit(1);
    }
    match args.last().unwrap().as_str() {
        "0" => {
            let layer0_string = get_file_content("./inputs/layer0.txt");
            layer0::decode_ascii85(layer0_string)
                .map(|output| write_file_content("./outputs/layer0.txt", &output))
                .unwrap();
        }
        "1" => {
            let layer1_string = get_file_content("./inputs/layer1.txt");
            layer0::decode_ascii85(layer1_string)
                .and_then(|layer1_input| layer1::decode(&layer1_input))
                .map(|output| write_file_content("./outputs/layer1.txt", &output))
                .unwrap();
        }
        "2" => {
            let layer2_string = get_file_content("./inputs/layer2.txt");
            layer0::decode_ascii85(layer2_string)
                .and_then(|layer2_input| layer2::decode(&layer2_input))
                .map(|output| write_file_content("./outputs/layer2.txt", &output))
                .unwrap();
        }
        "3" => {
            let layer3_string = get_file_content("./inputs/layer3.txt");
            layer0::decode_ascii85(layer3_string)
                .map(|layer3_input| layer3::decrypt(&layer3_input))
                .map(|output| write_file_content("./outputs/layer3.txt", &output))
                .unwrap();
        }
        "4" => {
            let layer4_string = get_file_content("./inputs/layer4.txt");
            layer0::decode_ascii85(layer4_string)
                .map(|layer4_input| layer4::get_data(&layer4_input))
                .map(|output| write_file_content("./outputs/layer4.txt", &output))
                .unwrap();
        }
        "5" => {
            let layer5_string = get_file_content("./inputs/layer5.txt");
            layer0::decode_ascii85(layer5_string)
                .map(|layer5_input| layer5::decrypt(&layer5_input))
                .map(|output| write_file_content("./outputs/layer5.txt", &output))
                .unwrap();
        }
        "6" => {
            let layer6_string = get_file_content("./inputs/layer6.txt");
            layer0::decode_ascii85(layer6_string)
                // .map(|layer6_input| layer6::execute(&layer6_input))
                .map(|output| write_file_content("./outputs/layer6.txt", &output))
                .unwrap();
        }
        _ => {
            eprintln!("Invalid layer");
            std::process::exit(1);
        }
    }
}
