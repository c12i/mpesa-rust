#[macro_export]
/// A quick util macro to create and populate a HashMap
/// 
/// # Example
/// ```
///  
///  let map = mpesa::map!(
///     String::from("access_token") => String::from("12345abcde"),
///     String::from("expires") => String::from("infinite")
///  );
///     
/// let mut hm = std::collections::HashMap::new();
/// hm.insert("access_token".to_string(), "12345abcde".to_string());
/// hm.insert("expires".to_string(), "infinite".to_string());
/// 
/// assert_eq!(map, hm);
/// ```
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
}