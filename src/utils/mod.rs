#![allow(unused)]
use std::collections::HashMap;
use std::error::Error;

#[macro_export]
macro_rules! map(
    ($($key:expr => $value:expr),+) => {
       {
           let mut m: ::std::collections::HashMap<_,_> = ::std::collections::HashMap::new();
           $(
               m.insert($key, $value);
           )*
           m
       }
    };
);

pub fn extract_auth_token(hm: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let token = hm.get("access_token").unwrap();
    Ok(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_macro() {
        let macro_map = map!(
            "one" => 1,
            "two" => 2
        );

        let mut test_map = HashMap::new();
        test_map.insert("one", 1);
        test_map.insert("two", 2);

        assert_eq!(macro_map, test_map);
    }

    #[test]
    fn test_extract_auth_token() {
       let map = map!(
           String::from("access_token") => String::from("12345abcde"),
           String::from("expires") => String::from("infinite")
       );

       assert_eq!(String::from("12345abcde"), extract_auth_token(&map).unwrap());
    }
}