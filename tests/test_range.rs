use nimbus_text_editor::split_range;

#[test]
fn test_split_range() {
    let ranges = [0..2, 2..6, 6..8];
    let res = split_range(&(0..8), &[2, 6]);
   
    assert_eq!(ranges.len(), res.len());
    for pair in ranges.iter().zip(res.iter()) {
        assert_eq!(pair.0, pair.1); 
    }
}

