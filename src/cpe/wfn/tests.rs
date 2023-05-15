use super::*;

#[test]
fn create_empty_wfn() {
    let new_wfn = Wfn::new();
    let empty_wfn = Wfn {
        part: AttributeValue::Unspecified,
    };
    assert_eq!(new_wfn, empty_wfn);
}

#[test]
fn display_empty_wfn() {
    let new_wfn = Wfn::new();
    let empty_wfn_string = String::from("wfn: []");
    assert_eq!(new_wfn.to_string(), empty_wfn_string);
}

#[test]
fn set_part_attribute_application_lower_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("a");
    if let AttributeValue::Value(p) = new_wfn.part {
        assert_eq!(p, Part::A);
    }
    else {
        panic!("Part attribute not assigned a value");
    }   
}

#[test]
fn set_part_attribute_application_upper_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("A");
    if let AttributeValue::Value(p) = new_wfn.part {
        assert_eq!(p, Part::A);
    }
    else {
        panic!("Part attribute not assigned a value");
    }   
}

#[test]
fn set_part_attribute_os_lower_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("o");
    if let AttributeValue::Value(p) = new_wfn.part {
        assert_eq!(p, Part::O);
    }
    else {
        panic!("Part attribute not assigned a value");
    }   
}

#[test]
fn set_part_attribute_os_upper_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("O");
    if let AttributeValue::Value(p) = new_wfn.part {
        assert_eq!(p, Part::O);
    }
    else {
        panic!("Part attribute not assigned a value");
    }   
}

#[test]
fn set_part_attribute_hardware_lower_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("h");
    if let AttributeValue::Value(p) = new_wfn.part {
        assert_eq!(p, Part::H);
    }
    else {
        panic!("Part attribute not assigned a value");
    }   
}

#[test]
fn set_part_attribute_hardware_upper_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("H");
    if let AttributeValue::Value(p) = new_wfn.part {
        assert_eq!(p, Part::H);
    }
    else {
        panic!("Part attribute not assigned a value");
    }   
}

#[test]
fn set_part_attribute_any_lower_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("any");
    assert_eq!(new_wfn.part, AttributeValue::Any);
}

#[test]
fn set_part_attribute_any_upper_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("ANY");
    assert_eq!(new_wfn.part, AttributeValue::Any);
}

#[test]
fn set_part_attribute_na_lower_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("na");
    assert_eq!(new_wfn.part, AttributeValue::Na);
}

#[test]
fn set_part_attribute_na_upper_case(){
    let mut new_wfn = Wfn::new();
    let _ = new_wfn.set_part("NA");
    assert_eq!(new_wfn.part, AttributeValue::Na);
}

#[test]
fn set_part_attribute_invalid_value(){
    let mut new_wfn = Wfn::new();
    let r = new_wfn.set_part("hardware");
    if let Err(x) = r {
        assert_eq!(x, CpeError::InvalidValueError);
    }
    else {
        panic!("set_part() did not correctly return an error");
    }
}