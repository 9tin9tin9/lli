use super::*;

#[test]
fn read_ltl(){
    let mut m = Mem::new();
    let s = "asdasd";
    for c in s.bytes() {
        m.nmem_allc(&[c as f64]);
    }
    m.nmem_allc(&[0f64; 2]);
    assert_eq!(m.read_ltl(-1).unwrap(), "asdasd");
}
