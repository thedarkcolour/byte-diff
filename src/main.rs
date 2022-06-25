use native_dialog::{FileDialog};
use std::io::stdin;


fn main() {
    let mut line: String = String::new();

    println!("Select mode:\n 0: TRACK\n 1: DIFF");
    let mode = read_num(&mut line);

    match mode {
        0 => {
            mode_track();
        }
        1 => {
            mode_diff();
        }
        _ => {
            println!("Mode {} is not valid", mode);
        }
    }

    println!("Press enter to exit...");
    stdin().read_line(&mut line).unwrap();
}

fn mode_diff() {
    println!("Select two files to compare...");
    
    let path_one = FileDialog::new()
        .show_open_single_file()
        .unwrap();
        
    let path_two = FileDialog::new()
        .show_open_single_file()
        .unwrap();
        
    if let Some(path_one) = path_one {
        if let Some(path_two) = path_two {
            let file_one = std::fs::read(path_one).expect("Unable to open the first file");
            let file_two = std::fs::read(path_two).expect("Unable to open the second file");
            
            let length = std::cmp::min(file_one.len(), file_two.len());
            
            println!("Scanning for differences");
            for i in 0 .. length {
                if file_one[i] != file_two[i] {
                    // , fileA[i], fileB[i]
                    //  - File A has value {:02X}, File B has value {:02X}
                    println!("{:X}", i);
                }
            }
            
            println!("Reached end of smallest file at {} bytes", length);
        }
    }
}

fn mode_track() {
    let mut target_value: u32;
    let mut line = String::new();
    let mut valid_locations: Vec<usize> = vec![];

    println!("What type of number are you looking for?\nDECIMAL\n0: u8 \n1: u16\n2: u32");
    let search_fn = match read_num(&mut line) {
        0 => {
            println!("Chosen 8 bit unsigned decimal");
            search_u8
        },
        _ => {
            println!("Invalid option. Defaulting to u8");
            search_u8
        },
    };

    loop {
        println!("Enter target value: ");
        target_value = read_num(&mut line);
    
        println!("Open current verison of the file...");
        let path = FileDialog::new()
            .show_open_single_file()
            .unwrap();
    
        if let Some(path) = path {
            let bytes = std::fs::read(&path).expect(&format!("Could not read selected file path: `{}`", &path.display().to_string()));
    
            search_fn(bytes, target_value, &mut valid_locations);
    
            println!("Found locations with value {}: ", target_value);
            for loc in &valid_locations {
                println!("{:X}", loc);
            }
        }
    }

}

fn read_num(line: &mut String) -> u32 {
    line.retain(|_| false);
    stdin().read_line(line).unwrap();
    line.trim().parse::<u32>().expect(&format!("Could not parse number input `{}`", line.trim()))
}

fn search_u8(bytes: Vec<u8>, target_value: u32, valid_locations: &mut Vec<usize>) {
    if valid_locations.is_empty() {
        for (loc, byte) in bytes.iter().enumerate() {
            if *byte as u32 == target_value {
                valid_locations.push(loc);
            }
        }
    } else {
        valid_locations.retain(|loc| bytes[*loc] as u32 == target_value);
    }
}