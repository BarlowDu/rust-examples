use std::str;

fn main(){
    
        for i in 0..1114112 {
        //for i in 19904..19968{
            let mut v: Vec<u8> = Vec::new();
            if i <= 127 {
                //1
                v.push(i as u8);
            } else if i > 127 && i <= 2047 {
                //2
                let mut b = (63 & i) | 128;
                v.insert(0, b as u8);
                b = i >> 6;
                if b > 0 {
                    b = (31 & b) | 192;
                    v.insert(0, b as u8);
                }
            } else if i > 2047 && i <= 65535 {
                //3
                let mut b = (63 & i) | 128;
                v.insert(0, b as u8);
                b = i >> 6;
                if b > 0 {
                    b = (63 & b) | 128;
                    v.insert(0, b as u8);
                    b = i >> 12;
                    if b > 0 {
                        b = (15 & b) | 224;
                        v.insert(0, b as u8);
                    }else{
                        v.insert(0, 224);
                    }
                }
            } else if i > 65536 && i <= 2097151 {
                //4
                let mut b = (63 & i) | 128; //b1
                v.insert(0, b as u8);
                b = i >> 6;
                if b > 0 {
                    //b2
                    b = (63 & b) | 128;
                    v.insert(0, b as u8);
                    b = i >> 12;
                    if b > 0 {
                        //b3
                        b = (63 & b) | 128;
                        v.insert(0, b as u8);
                        b = i >> 18;
                        if b > 0 {
                            //b4
                            b = (b & 7) | 240;
                            v.insert(0, b as u8);
                        } else {
                            v.insert(0, 240);
                        }
                    } else {
                        v.insert(0, 128);
                    }
                }
            }
            if v.len() <= 0 {
                continue;
            }
            let r = str::from_utf8(v.as_ref());
            match r {
                Ok(s) => {print!("{}", s);},
                Err(e)=> {}
            }

        }
}