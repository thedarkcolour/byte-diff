use native_dialog::{FileDialog};

fn main() {
    let pathA = FileDialog::new()
		.show_open_single_file()
		.unwrap();
		
	let pathB = FileDialog::new()
		.show_open_single_file()
		.unwrap();
		
	if let Some(pathA) = pathA {
		if let Some(pathB) = pathB {
			let fileA = std::fs::read(pathA).expect("Unable to open the first file");
			let fileB = std::fs::read(pathB).expect("Unable to open the second file");
			
			let length = std::cmp::min(fileA.len(), fileB.len());
			
			println!("Scanning for differences");
			for i in 0 .. length {
				if fileA[i] != fileB[i] {
					// , fileA[i], fileB[i]
					//  - File A has value {:02X}, File B has value {:02X}
					println!("{:X}", i);
				}
			}
			
			println!("Reached end of smallest file at {} bytes", length);
		}
	}
}
