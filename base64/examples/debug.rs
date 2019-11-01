extern crate base64;
use std::str;

fn main(){
    let v:&str="一在";
    let s=base64::to_base64(String::from(v).as_ref());
    let s1=base64::from_base64(s.as_ref());
    println!("{}",v.eq(&s1));
    println!("{}",s1);
}