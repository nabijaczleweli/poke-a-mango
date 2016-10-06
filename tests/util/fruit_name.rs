use poke_a_mango::util::{FRUITS, fruit_name};


#[test]
fn mango() {
    assert_eq!(fruit_name(&None), "Mango");
}

#[test]
fn not_mango() {
    for (i, &fruit) in FRUITS.iter().enumerate() {
        assert_eq!(fruit_name(&Some(i)), fruit);
    }
}

#[test]
#[should_panic]
fn oob() {
    fruit_name(&Some(FRUITS.len()));
}
