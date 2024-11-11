#[cfg(test)]
mod test {
    use roaring::RoaringTreemap;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test() {
        let mut treemap = RoaringTreemap::new();
        treemap.push(1u64);
        treemap.push(2u64);

        let mut bytes = vec![];
        treemap.serialize_into(&mut bytes).unwrap();
        assert_eq!(32, treemap.serialized_size());

        let mut file = File::create("/tmp/bitmap.dump").unwrap();
        let _ = file.write_all(&bytes);
    }
}
