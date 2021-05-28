use std::collections::HashMap;

pub struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<String, u32>,
}

// HashMap으로 처리 되도록 수정
impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        self.value.entry(arg.to_string()).or_insert(arg);

        arg
    }
}

// impl<T> Cacher<T>
//     where T: Fn(u32) -> u32 {
//     pub fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }
//
//     pub fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => {
//                 println!("exist value!");
//                 v
//             }
//             None => {
//                 println!("value not exist");
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }