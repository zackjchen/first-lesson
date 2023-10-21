#![allow(warnings)]

fn main() {
    level_one::fun_a_to_Z();
    level_one::level_two::fun_A_to_z()

}

mod level_one {

    pub fn fun_a_to_Z(){
        //a ～ Z 之间，开区间
        for i in ('Z' as u8 +1) as char..'a'{
            println!("{i}")
        }
    }

    pub mod level_two {
        pub fn fun_A_to_z() {
            for i in 'A'..('z' as u8 + 1) as char {
                println!("{i}")
            }
        }
    }
}