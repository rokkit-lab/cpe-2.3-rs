
//TODO: More advanced error handling

#[derive(Debug, PartialEq)]
pub struct WFN {
    part:       WFNOption<Part>,
    vendor:     WFNOption<WFNString>,
    product:    WFNOption<WFNString>,
    version:    WFNOption<WFNString>,
    update:     WFNOption<WFNString>,
    edition:    WFNOption<WFNString>,
    language:   WFNOption<WFNString>, //TODO: Make this RFC5646 compatible
    sw_edition: WFNOption<WFNString>,
    target_sw:  WFNOption<WFNString>,
    target_hw:  WFNOption<WFNString>,
    other:      WFNOption<WFNString>,  
}

impl WFN {

    // new() takes no arguments and returns an empty WFN.
    // Defined by Section 5.4.1 of [CPE23-N]
    pub fn new() -> WFN {
        WFN {
            part:       WFNOption::Unspecified,
            vendor:     WFNOption::Unspecified,
            product:    WFNOption::Unspecified,
            version:    WFNOption::Unspecified,
            update:     WFNOption::Unspecified,
            edition:    WFNOption::Unspecified,
            language:   WFNOption::Unspecified,
            sw_edition: WFNOption::Unspecified,
            target_sw:  WFNOption::Unspecified,
            target_hw:  WFNOption::Unspecified,
            other:      WFNOption::Unspecified,
        }
    }
}

// Similar to Option<T> but allows the ANY, NA and Unspecified values described in [CPE23-N]. 
#[derive(Debug, PartialEq)]
pub enum WFNOption<T>{
    Value(T),
    ANY,
    NA,
    Unspecified,
}


#[derive(Debug, PartialEq)]
pub enum Part {
    Application,
    OS,
    Hardware,
}

/// A string to be used as a string in an attribute-value string pair of a Well Formed CPE Name.
/// The value of a WFNString is guaranteed to be a valid attribute-value string as defined by [CPE23-N]. 
#[derive(Debug, PartialEq)]
pub struct WFNString {
    value:      String,
}

impl TryFrom<&str> for WFNString {
    type Error = &'static str;

    fn try_from(wfn_str: &str) -> Result<Self, Self::Error> {
        
        // WFN attribute-value strings cannot be empty [CPE23-N-5.3.2]
        if wfn_str.is_empty() {
            Err("Empty String")   
        }
        // WFN attribute-value strings cannot contain whitespace [CPE23-N-5.3.2-2]
        else if is_whitespace(wfn_str) {
            Err("String contains whitespace") 
        }
        // WFN attribute-value strings cannot only contain a quoted hyphen [CPE23-N-5.3.2-6] 
        else if is_quoted_hyphen(wfn_str) {
            Err("String is quoted hypen") 
        }
        else {
            Ok(WFNString { value: String::from(wfn_str)})
        }
    }
}

// helper functions
fn is_whitespace(str: &str) -> bool {
    str.contains(char::is_whitespace)
}

fn is_quoted_hyphen(str: &str) -> bool {
    if str == r"\-" {
        true
    }
    else {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_wfn() {
        let new_wfn = WFN::new();
        let old_wfn = WFN {
            part:       WFNOption::Unspecified,
            vendor:     WFNOption::Unspecified,
            product:    WFNOption::Unspecified,
            version:    WFNOption::Unspecified,
            update:     WFNOption::Unspecified,
            edition:    WFNOption::Unspecified,
            sw_edition: WFNOption::Unspecified,
            target_sw:  WFNOption::Unspecified,
            target_hw:  WFNOption::Unspecified,
            language:   WFNOption::Unspecified,
            other:      WFNOption::Unspecified,
        };
        assert_eq!(new_wfn, old_wfn);
    }

    #[test]
    fn create_empty_wfn_string() {
        let wfn_string = WFNString::try_from("");
        assert_eq!(wfn_string, Err("Empty String"));
    }

    #[test]
    fn create_wfn_string() {
        let value = "test_string";
        let wfn_string = WFNString::try_from(value).unwrap();
        assert_eq!(wfn_string.value, value);
    }

    #[test]
    fn wfn_string_whitespace_space() {
        let str = "hello world";
        let wfn_string = WFNString::try_from(str);
        assert_eq!(wfn_string, Err("String contains whitespace"));
    }

    #[test]
    fn wfn_string_whitespace_new_line() {
        let str = "hello_world\n";
        let wfn_string = WFNString::try_from(str);
        assert_eq!(wfn_string, Err("String contains whitespace"));
    }

    #[test]
    fn wfn_string_whitespace_tab() {
        let str = "hello\tworld";
        let wfn_string = WFNString::try_from(str);
        assert_eq!(wfn_string, Err("String contains whitespace"));
    }

    #[test]
    fn wfn_string_whitespace_carriage_return() {
        let str = "hello_world\r";
        let wfn_string = WFNString::try_from(str);
        assert_eq!(wfn_string, Err("String contains whitespace"));
    }

    #[test]
    fn wfn_string_whitespace_form_feed() {
        // \u{C} == \f - rust doesn't support \f directly
        let str = "hello_world\u{C}"; 
        let wfn_string = WFNString::try_from(str);
        assert_eq!(wfn_string, Err("String contains whitespace"));
    }

    #[test]
    fn wfn_string_whitespace_vertical_tab() {
        // \u{B} == \v - rust doesn't support \v directly
        let str = "hello_world\u{B}"; 
        let wfn_string = WFNString::try_from(str);
        assert_eq!(wfn_string, Err("String contains whitespace"));
    }

    #[test]
    fn wfn_string_quoted_hypen() {
        let str = r"\-";
        let wfn_string = WFNString::try_from(str);
        assert_eq!(wfn_string, Err("String is quoted hypen"));
    }
}
