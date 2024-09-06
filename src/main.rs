use clap::{Arg, Command};
use rxing::aztec::AztecWriter;
use rxing::common::BitMatrix;
use rxing::BarcodeFormat;
use rxing::Writer;
use rxing::EncodeHintType;
use rxing::EncodeHintValue;

fn main() {
    let matches = Command::new("Aztec Code Generator")
        .version("1.0")
        .author("Nils Schwabe <nils@schwabe.ws>")
        .about("Generates an Aztec code and prints it to the command line.")
        .arg(
            Arg::new("input")
                .help("The input data to encode in the Aztec code")
                .required(true)
                .index(1),
        )
        .get_matches();

    let data = matches.get_one::<String>("input").unwrap();
    let writer = AztecWriter {};
    let hints = rxing::EncodingHintDictionary::from([
        (EncodeHintType::CHARACTER_SET, EncodeHintValue::CharacterSet("UTF-8".into()))
    ]);
    let bit_matrix: BitMatrix = writer.encode_with_hints(data, &BarcodeFormat::AZTEC, 0, 0, &hints).unwrap();

    for y in (0..bit_matrix.height()).step_by(2) {
        for x in 0..bit_matrix.width() {
            if bit_matrix.get(x, y) {
                if bit_matrix.get(x, y+1) {
                    print!("█");
                } else {
                    print!("▀");
                }
            } else {
                if bit_matrix.get(x, y+1) {
                    print!("▄");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
}

