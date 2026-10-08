use bare::{BareReader, BareWriter};

#[test]
fn writes_and_reads_graph() {
    let path = "test.bare";

    let mut writer = BareWriter::create(path).unwrap();

    writer.add_vertex(0, &[1, 2]).unwrap();
    writer.add_vertex(1, &[0, 2]).unwrap();
    writer.add_vertex(2, &[0]).unwrap();

    writer.finish().unwrap();

    let mut reader = BareReader::open(path).unwrap();

    assert_eq!(reader.neighbors(0).unwrap(), vec![1, 2]);
    assert_eq!(reader.neighbors(1).unwrap(), vec![0, 2]);
    assert_eq!(reader.neighbors(2).unwrap(), vec![0]);

    let range = reader.adjacency_range(1).unwrap();

    println!("Vertex 1:");
    println!("  offset: {}", range.offset);
    println!("  length: {}", range.length);

    let bytes = std::fs::read(path).unwrap();

    println!("File size: {} bytes", bytes.len());

    println!("File bytes:");
    for (i, chunk) in bytes.chunks(16).enumerate() {
        print!("{:04x}: ", i * 16);

        for byte in chunk {
            print!("{:02x} ", byte);
        }

        println!();
    }

    std::fs::remove_file(path).unwrap();
}
