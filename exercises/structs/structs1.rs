// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

struct ColorUnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let name = "green".to_string();
        let hex = "#00FF00".to_string();
        let green = ColorClassicStruct { name, hex };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // For more fun, use the field initialization shorthand.
        let name = "green".to_string();
        let hex = "#00FF00".to_string();
        let green = ColorTupleStruct(name, hex);

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        let green = ColorUnitStruct;

        if let ColorUnitStruct = green {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
