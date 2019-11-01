use std::str;
use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;

const BASE_STR:&str="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

lazy_static!{
    
    static ref BASE_MAP: HashMap<char, u8> = {
        let mut m: HashMap<char, u8> = HashMap::new();
        m.insert('A', 0);
        m.insert('B', 1);
        m.insert('C', 2);
        m.insert('D', 3);
        m.insert('E', 4);
        m.insert('F', 5);
        m.insert('G', 6);
        m.insert('H', 7);
        m.insert('I', 8);
        m.insert('J', 9);
        m.insert('K', 10);
        m.insert('L', 11);
        m.insert('M', 12);
        m.insert('N', 13);
        m.insert('O', 14);
        m.insert('P', 15);
        m.insert('Q', 16);
        m.insert('R', 17);
        m.insert('S', 18);
        m.insert('T', 19);
        m.insert('U', 20);
        m.insert('V', 21);
        m.insert('W', 22);
        m.insert('X', 23);
        m.insert('Y', 24);
        m.insert('Z', 25);
        m.insert('a', 26);
        m.insert('b', 27);
        m.insert('c', 28);
        m.insert('d', 29);
        m.insert('e', 30);
        m.insert('f', 31);
        m.insert('g', 32);
        m.insert('h', 33);
        m.insert('i', 34);
        m.insert('j', 35);
        m.insert('k', 36);
        m.insert('l', 37);
        m.insert('m', 38);
        m.insert('n', 39);
        m.insert('o', 40);
        m.insert('p', 41);
        m.insert('q', 42);
        m.insert('r', 43);
        m.insert('s', 44);
        m.insert('t', 45);
        m.insert('u', 46);
        m.insert('v', 47);
        m.insert('w', 48);
        m.insert('x', 49);
        m.insert('y', 50);
        m.insert('z', 51);
        m.insert('0', 52);
        m.insert('1', 53);
        m.insert('2', 54);
        m.insert('3', 55);
        m.insert('4', 56);
        m.insert('5', 57);
        m.insert('6', 58);
        m.insert('7', 59);
        m.insert('8', 60);
        m.insert('9', 61);
        m.insert('+', 62);
        m.insert('/', 63);
        m
    };
}

pub fn to_base64(value: &str) -> String {
    //let base = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = value.as_bytes();
    let mut i = 0;
    let mut left = 0;
    let mut result: String = String::from("");
    let mut current:u8=0;
    let mut index:u8=0;
    while i < bytes.len() {
        current=bytes[i];
        if left==0{
            index=current>>2;
            i = i + 1;
            left=-2;
        }
        else if left==-2{
            let prev=bytes[i-1];
            index=((prev&0x3)<<4)|((current&0xF0)>>4);
            i = i + 1;
            left=-4;
        }
        else if left==-4{
            let prev=bytes[i-1];
            index=((prev&0xF)<<2)|((current&0xC0)>>6);
            left=2;
        }
        else if left==2{
            index=current&0x3F;
            i = i + 1;
            left=0;
        }
        let item=BASE_STR.get((index as usize)..((index+1) as usize));
        match item{
            Some(c)=>result.push_str(c),
            None=>{}
        }
    }
    index=255;
    if left==-2{
        index=(current&0x3)<<4;
    }
    else if left==-4{
        index=(current&0xF)<<2;
    }/*else if left==2{
        index=current&0x3F;
    }*/
    if index!=255{

        let item=BASE_STR.get((index as usize)..((index+1) as usize));
        match item{
            Some(c)=>result.push_str(c),
            None=>{}
        }
        let m=result.len()%4;
        for i in 0..m{
            result.push('=');
        }

    }
    //println!("left:{}",left);
    //println!("{}",result);
    //str::from_utf8(v: &[u8])
    return result;
}

pub fn from_base64(code:&str)->String{
    let mut result:Vec<u8>=Vec::new();
    let mut left=0;
    let mut b=0;
    for ch in code.chars(){
        let mut n:u8=0;
        let  i=BASE_MAP.get(&ch);
        match i{
            Some(m)=>n=*m,
            None=>{break;}
        }
        if left==0{
            b=n<<2;
            left=2;
        }else if left==2{
            b=b|(n>>4);
            result.push(b);
            b=n<<4;
            left=4;
        }else if left==4{
            b=b|n>>2;
            result.push(b);
            b=(n&3)<<6;
            left=6;
        }else if left==6{
            b=b|n;
            result.push(b);
            b=0;
            left=0;
        }
    }
    let rs=str::from_utf8(result.as_ref()).unwrap();
    return String::from(rs);
}

