use std::io::Error;



fn change(value: i32) -> Result<i32, Error> {
    if value < 0 {
        Err(Error::new(std::io::ErrorKind::InvalidInput, "Negative value provided"))
    } else {
        Ok(value)
    }
}

#[test]
fn zero_test(){
    assert_eq!(change(0).unwrap(), 0);
}


#[test]
fn twelve_test() {
    assert_eq!(change(12).unwrap(), 3);
}

#[test]
fn four_sixty_eight_test() {
    assert_eq!(change(468).unwrap(), 11);
}

#[test]
fn one_two_three_four_five_six_test() {
    assert_eq!(change(123456).unwrap(), 254);
}
