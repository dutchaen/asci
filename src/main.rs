use core::panic;
use std::io::Write;
use std::{collections::HashSet, fs::File};

use rand::Rng;
use image::ImageReader;

enum Languages {
    Python,
    C
}


fn main() {

    let mut rng = rand::thread_rng();
    println!("asci: automatic ascii art");

    eprint!("[?] Where is the image you are trying to convert? >> ");
    let path = read_input::<String>()
        .unwrap();

    eprint!("[?] Would you like the colors to be inverted? [y/n] >> ");
    let color_invert_text = read_input::<String>()
        .unwrap()
        .to_ascii_lowercase();

    let wants_invert = color_invert_text.starts_with('y');

    let ascii_art = image_to_ascii(path, wants_invert);

    println!("[!] Successfully converted.");

    println!();


    println!("[     Options    ]");
    println!("[0] Print the ASCII art");
    println!("[1] Write the ASCII art into a text file");
    println!("[2] Create a readable Python or C file that will print the ASCII art");
    println!("[3] Create a sophisticated Python or C file that will print the ASCII art (+ego boost guaranteed)");

    println!();

    eprint!("[?] Select an option >> ");
    let option = read_input::<u32>()
        .expect("option was invalid");

    if option > 3 {
        println!("[x] invalid option...");
        std::process::exit(-1);
    }

    println!();

    match option {
        0 => println!("{}", ascii_art),
        1 => {
            let mut f = File::create_new("ascii_art.txt")
                .expect("expected to create ascii_art.txt text file");

            writeln!(f, "{}", ascii_art).unwrap();

            println!("[!] Written to 'ascii_art.txt'.");
        },
        2 => {
            let lang = select_lang();

            match lang {
                Languages::Python => {
                    let mut code = String::new();

                    for line in ascii_art.lines() {
                        let python_line = format!("print(\"{}\")\n", line);
                        code += &python_line;
                    }

                    let mut f = File::create("code.py")
                        .expect("");

                    writeln!(f, "{}", code).unwrap();
                    println!("[!] Written to 'code.py'.");
                }
                Languages::C => {
                    let mut code = String::new();
                    code.push_str("#include <stdio.h>\n");
                    code.push_str("#include <stdlib.h>\n");

                    code.push('\n');
                    code.push_str("int main(int argc, char** argv) {\n");

                    for line in ascii_art.lines() {
                        let c_line = format!("\tprintf(\"{}\\n\");\n", line);
                        code += &c_line;
                    }

                    code.push_str("\treturn 0;\n");

                    code.push_str("}\n");

                    let mut f = File::create("code.c")
                        .expect("");

                    writeln!(f, "{}", code).unwrap();
                    println!("[!] Written to 'code.c'.");
                }

                
            };
            
        },
        3 => {
            let lang = select_lang();

            match lang {
                Languages::Python => {
                    let mut code = String::new();
                    code += "import base64\r\n";
                    code += "\r\n";
                    code += "secret_passcode = 'ZGVjcnlwdF9tZXNzYWdlID0gcHJpbnQ='\r\n";
                    code += "decryption_code = compile(base64.b64decode(secret_passcode), 'null', 'exec')\r\n";
                    code += "exec(decryption_code)\r\n";

                    code += "\r\n";

                    let mut variable_names = vec![];
                    for (line_index, line) in ascii_art.lines().enumerate() {

                        variable_names.push(format!("memory_0x{:x}", line_index * line.len()));

                        code += &format!("memory_0x{:x} = ''.join([\n", line_index * line.len());
                        for (char_index, c) in line.chars().enumerate() {
                            let key = rng.gen_range(4..12);
                            let char_as_i32 = c as i32;

                            let encoded_char_as_i32 = char_as_i32 << key;

                            let show_as_binary = rng.gen_bool(0.125);

                            let should_add_tab = char_index % 4 == 0;
                            if should_add_tab {
                                code.push('\t');
                            }

                            if show_as_binary {
                                code += &format!("chr(0b{:b} >> 0x{:x})", encoded_char_as_i32, key);
                            }
                            else {
                                code += &format!("chr(0x{:x} >> 0x{:x})", encoded_char_as_i32, key);
                            }

                            let has_4_chars_written_on_this_line = (char_index + 1) % 4 == 0;
                            let no_more_chars_left = char_index + 1 == line.len();


                            if has_4_chars_written_on_this_line && no_more_chars_left {
                                code += "\r\n])\r\n";
                            }
                            else if has_4_chars_written_on_this_line {
                                code += ",\r\n";
                            }
                            else if no_more_chars_left {
                                code += "\r\n])";
                            }
                            else {
                                code += ", ";
                            }

                        }

                        code += "\n";
                    }

                    code += "\n\n\n";

                    for variable_name in variable_names.iter() {
                        code += &format!("decrypt_message({})\r\n", variable_name);
                    }

                    let mut f = File::create("code.py")
                        .expect("");

                    writeln!(f, "{}", code)
                        .unwrap();
                    println!("[!] Written to 'code.py'.");
                }
                Languages::C => {
                    let mut code = String::new();
                    code.push_str("#include <stdio.h>\n");
                    code.push_str("#include <stdlib.h>\n");
                    code.push_str("#include <stddef.h>\n");

                    code.push('\n');
                    code.push_str("int main(int argc, char** argv) {\n");

                    let mut variable_names = vec![];
                    for (_, line) in ascii_art.lines().enumerate() {

                        let name = format!("_0x{:x}", rng.gen_range(1_000_000..=4_000_000));
                        variable_names.push(name.clone());

                        code += &format!("\tconst wchar_t {}[0b{:b}] = {{\n", name, line.len()+1);
                        for (char_index, c) in line.chars().enumerate() {
                            let key = rng.gen_range(4..12);
                            let char_as_i32 = c as i32;

                            let encoded_char_as_i32 = char_as_i32 << key;

                            let show_as_binary = rng.gen_bool(0.125);

                            let should_add_tab = char_index % 4 == 0;
                            if should_add_tab {
                                code.push_str("\t\t");
                            }

                            if show_as_binary {
                                code += &format!("(wchar_t)(0b{:b} >> 0x{:x})", encoded_char_as_i32, key);
                            }
                            else {
                                code += &format!("(wchar_t)(0x{:x} >> 0x{:x})", encoded_char_as_i32, key);
                            }

                            let has_4_chars_written_on_this_line = (char_index + 1) % 4 == 0;

                            if has_4_chars_written_on_this_line {
                                code += ",\r\n";
                            }
                            else {
                                code += ", ";
                            }

                        }

                        code += "\t\t(wchar_t)(0x00)\r\n\t};\n\n";

                    }

                    code += "\n\n\n";

                    for variable_name in variable_names.iter() {
                        code += "\twprintf(L\"%ls\\n\", ";
                        code += &format!("{});\r\n", variable_name);
                    }

                    code.push_str("\treturn 0;\n");

                    code.push_str("}\n");

                    let mut f = File::create("code.c")
                        .expect("");

                    writeln!(f, "{}", code)
                        .unwrap();
                    println!("[!] Written to 'code.c'.");
                }

                
            };
        }
        invalid_option => panic!("invalid option: {}", invalid_option)
    }

}

fn image_to_ascii(path: String, wants_invert: bool) -> String {
    let reader = ImageReader::open(path)
    .unwrap();

    let decoded_image = reader.decode()
        .expect("image should be a decodable format");

    let filter = image::imageops::FilterType::Gaussian;    

    let resized_image = decoded_image
        .resize_exact(128, 64, filter)
        .grayscale();

    let rgb_image = resized_image.to_rgb8();

    let mut builder = String::new();

    let mut colors_used: HashSet<[u8;3]> = HashSet::new();

    for (_, pixels) in rgb_image.enumerate_rows() {
        for (_, _, pixel) in pixels {

            let raw_pixels = &pixel.0;
            colors_used.insert(raw_pixels.clone());

            let decoded_char = match wants_invert { 
                true => decode_pixel_inverted(raw_pixels[0]),
                false => decode_pixel(raw_pixels[0])
            };

            builder.push(decoded_char);
        }

        builder.push('\n');
    }

    return builder;
}

fn select_lang() -> Languages {
    println!("[     Languages    ]");
    println!("[0] Python        [py]");
    println!("[1] C             [ c]");
    println!();

    eprint!("[?] Choose the language [py/c] >> ");
    let language_text = read_input::<String>()
        .unwrap()
        .to_ascii_lowercase();

    let lang = match language_text.as_str() {
        "py" => Languages::Python,
        "c"  => Languages::C,
        _ => {
            println!("[!] Language given was not a correct language. Defaulting to Python...");
            Languages::Python
        }
    };

    return lang;
}

fn decode_pixel(pixel: u8) -> char {
    if pixel < 50 {
        ' '
    }
    else if pixel < 100 {
        '`'
    }
    else {
        '•'
    }
}

fn decode_pixel_inverted(pixel: u8) -> char {
    if pixel > 100 {
        ' '
    }
    else if pixel > 50 {
        '`'
    }
    else {
        '•'
    }
}

fn read_input<T: std::str::FromStr>
    () -> Result<T, Box<dyn std::error::Error>> {

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    match input.parse() {
        Ok(value) => Ok(value),
        Err(_) => Err("could not be parsed".into())
    }
}