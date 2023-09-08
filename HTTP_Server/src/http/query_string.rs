use std::collections::HashMap;
// Key a has value 1, key b has value 2, key c with empty value
// key d with empty string, e has value ==, and d again have two more
// values. means for key d, we would have an array of values
// a=1&b=2&c&d=&e===&d=7&d=abc
pub struct QueryString<'buf>{
    // data: HashMap<&'buf str , &'buf str>,
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf>{
    Single(&'buf str),
    // Vector is a heap allocated dynamic array
    Multiple(Vec<&'buf str>),
}
impl<'buf> QueryString<'buf>{
    pub fn get(&self, key:&str) -> Option<&Value>{
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";
            
            if let Some(i) = sub_str.find('='){
                key = &sub_str[..i];
                val = &sub_str[i+1..];

            }
            // If it is multiple, push it to the vector
            data.entry(key)
            .and_modify(|existing: &mut Value | match existing{
                Value::Single(prev_val) => {
                    // let mut vec = Vec::new();
                    // vec.push(val);
                    // vec.push(prev_val);
                    // The other way of doing the work of previous 3 lines;
                    let mut vec = vec![prev_val, val];
                    *existing =  Value::Multiple(vec);
                }
                Value::Multiple(vec) => vec.push(val)
            })
            .or_insert(Value::Single(val));
        }
        QueryString { data}
        
    }
}