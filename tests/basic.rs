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

    assert!(range.length > 0);

    std::fs::remove_file(path).unwrap();
}
