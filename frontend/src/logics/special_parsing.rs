pub fn parse_from_str_to_u32(string : &&str)->u32{
        let number_from_string_1:&str = *string;
        let number_from_string_2: u32 = number_from_string_1.parse().unwrap(); 
        number_from_string_2
    }