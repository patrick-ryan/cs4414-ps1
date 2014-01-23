use std::os;
use std::io::File;

fn main() {
	let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <file1> <file2>", args[0]); 
    } else {
    	let file1 = args[1].clone();
    	let file2 = args[2].clone();
    	let path1 = Path::new(file1.clone());
    	let path2 = Path::new(file2.clone());
        let share1 = File::open(&path1);
        let share2 = File::open(&path2);

        match (share1, share2) {
        	(Some(mut s1), Some(mut s2)) => {
        		let s1_bytes: ~[u8] = s1.read_to_end();
        		let s2_bytes: ~[u8] = s2.read_to_end();
        		let msg_file 
                    = File::create(&Path::new("msg"));

                match (msg_file) {
                	Some(msg) => join(s1_bytes, s2_bytes, msg),
                	None => fail!("Error opening output file!"),
                }
                
        	},
        	(_, _) => fail!("Error opening input files!"),
        }
    }
	
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
		ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(s1_bytes: &[u8], s2_bytes: &[u8], mut msg_file: File) {
	let decrypted_bytes = xor(s1_bytes, s2_bytes);
	msg_file.write(decrypted_bytes);
}