extern crate cdg;

use std::fs::File;

fn main() {
    let filename = std::env::args().nth(1).expect("Usage: $0 filename");
    let file = File::open(filename).unwrap();
    let mut scsi = cdg::SubchannelStreamIter::new(file);

    while let Some(sector) = scsi.next() {
        for cmd in sector {
            println!("{:?}", cmd);
        }
        println!("---");
    }
}
