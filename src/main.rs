#![allow(warnings)]

fn main() {
    level_one::fun_a_to_Z();
    level_one::level_two::fun_z_to_A();
}

mod level_one {
    pub fn fun_a_to_Z(){
        for i in 65_u8..123_u8 {
            if i>90 && i<97 { continue }
            println!("{}",i as char)
        }
    }
    pub mod level_two{
        pub fn fun_z_to_A(){
            for i in (65_u8..123_u8).rev() {
                if i>90 && i<97 { continue }
                println!("{}",i as char)
            }
        }
    }
}