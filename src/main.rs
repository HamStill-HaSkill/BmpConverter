use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_from = &args[1];
    let convert_option = &args[2];
    let file_to = &args[3];

    // show_bytes(&file_from);

    if convert_option == "zip" {
        to_bmp(&file_from, &file_to);
    } else if convert_option == "unzip" {
        from_bmp(&file_from, &file_to);
    }
}

fn show_bytes(name: &str) {
    let file_name = &format!("src/build/{name}");
    let mut file = File::open(file_name).unwrap();
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    print!("{file_name} ");
    println!("{} bytes read", contents.len());

    let mut i = 0;
    for byte in contents {
        if i <= 62 {
            print!("0x{:02x}, ", byte);
        }
        i += 1
    }
}

fn from_bmp(name: &str, new_name: &str) {
    let file_name = &format!("src/build/{name}");
    let new_file_name = &format!("src/build/{new_name}");
    let mut file = File::open(file_name).unwrap();
    let mut contents = Vec::new();
    let mut new_contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();
    let mut i = 0;
    for byte in contents {
        if i > 61 {
            new_contents.push(byte);
        }
        i += 1;
    }

    let mut new_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(new_file_name)
        .unwrap();

    new_file.write_all(&new_contents).unwrap();
}

fn to_bmp(name: &str, new_name: &str) {
    let file_name = &format!("src/build/{name}");
    let new_file_name = &format!("src/build/{new_name}");
    let mut file = File::open(file_name).unwrap();
    let mut contents = Vec::new();
    let mut new_contents: Vec<u8> = [
        0x42, 0x4d, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x28,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, 0x00,
    ]
    .to_vec();

    file.read_to_end(&mut contents).unwrap();

    let file_size: u32 = 1024 as u32;
    let new_file_size: u32 = new_contents.len() as u32;

    let mut file_size_bytes = file_size.to_be_bytes();
    let mut new_file_size_bytes = (file_size + new_file_size).to_be_bytes();
    let mut size_in_pixels = file_size.to_be_bytes();

    file_size_bytes.reverse();
    new_file_size_bytes.reverse();
    size_in_pixels.reverse();

    println!("{file_name} ");
    println!("{} bytes read", contents.len());

    let mut i = 2;
    for byte in new_file_size_bytes {
        new_contents[i] = byte;
        i += 1;
    }

    i = 18;
    for byte in size_in_pixels {
        new_contents[i] = byte;
        i += 1;
    }

    i = 22;
    for byte in size_in_pixels {
        new_contents[i] = byte;
        i += 1;
    }

    let mut i = 34;
    for byte in file_size_bytes {
        new_contents[i] = byte;
        i += 1;
    }

    for byte in contents {
        new_contents.push(byte);
    }

    let mut new_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(new_file_name)
        .unwrap();

    new_file.write_all(&new_contents).unwrap();
}
