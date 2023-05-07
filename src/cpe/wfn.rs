//! A submodule providing Well Formed CPE Names (WFN) for the seiber CPE module.
//! 
//! Well Formed CPE Names (WFN) are defined by NIST Interagency Report 7695 *Common Platform Enumeration:
//! Naming Specification Version 2.3*. This standard is refered to as [CPE23-N] throughout documentation.
//! WFN describe a data model that can be used to describe hardware and software product classes. WFN themselves
//! are not intended to be machine readable, but form a basis from which machine readible product identifiers can
//! be created. The product identifiers can be used in conjuction with other cybersecurity tools, such as
//! Common Vulnerability Enumeration (CVE), to determine security information about the products.
//! 
//! Although the CPE Specification states that WFN are only a *'logical construct'* and does not require 
//! that CPE-conformant tools manipulate WFN-like data structures internally, WFN data structures are used
//! in seiber as the primary CPE data structure.
//! 
//! The seiber implimentation of WFN should only differ from the specification in \[CPE23-N] where absolutely
//! necessary to maintain idiomatic rust as far is practicable.

use super::error::CpeError;
use std::fmt;

/// A struct representing a Well Formed CPE Name (WFN) as defined in [CPE23-N]. WFN are defined as attribute-value
/// pairs.
#[derive(Debug, PartialEq)] 
pub struct Wfn {
    part: AttributeValue<Part>,
}

impl Wfn {
    /// Returns an empty WFN i.e. a WFN with all attributes having an unspecified value.
    pub fn new() -> Wfn {
        Wfn { part: AttributeValue::Unspecified }
    }

    /// A setter function to assign a value to the part attribute. Returns a blank Ok variant if the assignment
    /// was successful or an error if an incorrect value was assigned.
    pub fn set_part(&mut self, p: &str) -> Result<(), CpeError>{
        match p {
            "a" | "A" => {
                self.part = AttributeValue::Value(Part::A);
                Ok(())
            },
            "o" | "O" => {
                self.part = AttributeValue::Value(Part::O);
                Ok(())
            },
            "h" | "H" => {
                self.part = AttributeValue::Value(Part::H);
                Ok(())
            },
            _ => Err(CpeError::InvalidValueError)
        }
    }

    /// A getter function to get the value assigned to the part attribute. Returns the attribute value converted to a string
    /// wrapped in an Option. Unspecified attribute return None.
    pub fn get_part(self) -> Option<String> {
        // This match code is likely to be the same for most attributes. Can probably move this to a standalone function.
        match self.part {
            AttributeValue::Unspecified => None,
            AttributeValue::ANY => Some(String::from("ANY")),
            AttributeValue::NA => Some(String::from("NA")),
            AttributeValue::Value(x) => Some(x.to_string()),
        }
    } 
}

impl fmt::Display for Wfn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Opening of wfn string representation
        let mut s = String::from("wfn: ["); // TODO: Add listing of attributes and values

        // Closing of wfn string representation
        s.push_str("]");
        write!(f, "{}", s)
    }
}

/// An enumeration representing the values that can be assigned to an attribute.
#[derive(Debug, PartialEq)]
enum AttributeValue<T> {
    /// An attribute that has not been assigned a value. An unspecified attribute is processed in most functions
    /// as if it had been assigned the value of ANY. However, an unspecified attribute will not be listed when
    /// the WFN is displayed as a string. The attribute will still be displayed correctly when bound.
    Unspecified,
    /// The logical value of ANY or 'any value'. An attribute should be assined the value of ANY when
    /// there are no restrictions on acceptable values for the attribute [CPE23-N].  
    ANY,
    /// The logical value of NA or 'not applicable/not used'. An attribute should be assigned the value of NA
    /// when there is no legal or meaningful values for the attribute.
    NA,
    /// Holds the value of an attribute when an attribute has been assigned a value.
    Value(T),
}


/// An enumeration representing the part attribute of a WFN. The part attribute is implemented as an
/// enumeration rather than as a string with defined values as specified in [CPE23-N] since the enumeration
/// is more idiomatic. Similarly, [CPE23-N] specifies the part values in lower case. Upper case has been used
/// here to remain idiomatic.
#[derive(Debug, PartialEq)]
enum Part {
    /// Application
    A,
    /// Operating System
    O,
    /// Hardware
    H,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::A => write!(f, "a"), // [CPE23-N] defines these as lower case
            Part::O => write!(f, "o"),
            Part::H => write!(f, "h"),
        }
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(new_wfn.part, AttributeValue::ANY);
    }
}

