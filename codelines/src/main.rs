use regex::Regex;
use std::{
    env, fs,
    io::{Read},
    path::Path,
};

struct CollectModel {
    filesize: u64,
    filecount: u64,
    linecount: u64,
}

fn main() {
    let args = env::args();
    if args.len() < 2 {
        println!("请输入查询路径");
        return;
    }
    let mut arguments: Vec<String> = Vec::new();
    args.for_each(|arg| arguments.push(arg));
    let root = Path::new(arguments[1].as_str());

    let mut reg_file:Regex;
    if  arguments.len() > 2 {
        if let Ok(r) = Regex::new(arguments[2].as_str()) {
            reg_file = r;
        } else {
            println!("文件名匹配模式有误");
            return;
        }
    }
    else{
        reg_file=Regex::new("\\.*").unwrap();
    }
    let mut result = CollectModel {
        filesize: 0,
        filecount: 0,
        linecount: 0,
    };
    let md = fs::metadata(root);
    match md {
        Ok(md) => {
            let childr = collect(md, root,&reg_file);
            result.filecount += childr.filecount;
            result.filesize += childr.filesize;
            result.linecount += childr.linecount;
        }
        Err(_) => {
            println!("文件或者文件夹不存在!");
            return;
        }
    }

    println!("filecount:{}", result.filecount);
    println!("linecount:{}", result.linecount);
    println!("filesize:{}({})", convert_size(result.filesize),result.filesize);
}

fn collect<P: AsRef<Path>>(m: fs::Metadata, path: P,reg_file:&Regex) -> CollectModel {
    if m.is_file() {
        collect_file(m, path)
    } else {
        collect_dir(m, path,reg_file)
    }
}

fn collect_file<P: AsRef<Path>>(m: fs::Metadata, path: P) -> CollectModel {
    let mut result = CollectModel {
        filesize: 0,
        filecount: 0,
        linecount: 0,
    };
    result.filecount = 1;
    result.filesize = m.len();
    if let Some(count) = read_line_count(path) {
        result.linecount = count;
    }
    return result;
}

fn collect_dir<P: AsRef<Path>>(_m: fs::Metadata, path: P,reg_file:&Regex) -> CollectModel {
    let mut result = CollectModel {
        filesize: 0,
        filecount: 0,
        linecount: 0,
    };
    if let Ok(dirs) = fs::read_dir(path) {
        for dir in dirs {
            if let Ok(d) = dir {
                let p=d.path();
                if let Ok(md) = fs::metadata(d.path()) {
                    if md.is_file() {                        
                        let filename=p.file_name().unwrap().to_str().unwrap();
                        if reg_file.is_match(filename)==false{
                            continue;
                        }
                        let childr = collect_file(md, d.path());
                        result.filecount += childr.filecount;
                        result.filesize += childr.filesize;
                        result.linecount += childr.linecount;
                        println!("{},{},{}",p.to_str().unwrap(),childr.filesize,childr.linecount);
                    } else {
                        let childr = collect_dir(md, d.path(),reg_file);
                        result.filecount += childr.filecount;
                        result.filesize += childr.filesize;
                        result.linecount += childr.linecount;
                    }
                }
            }
        }
    }
    return result;
}

fn read_line_count<P: AsRef<Path>>(path: P) -> Option<u64> {
    let file = fs::File::open(path);
    if let Ok(mut f) = file {
        let mut buf = String::new();
        let sall = f.read_to_string(&mut buf);
        if let Ok(_) = sall {
            return Some(buf.lines().count() as u64);
        } else {
            return None;
        }
    }
    return None;
}

fn convert_size(s:u64)->String{
    let s1=(s as f64)/1024f64;
    if s1<1024f64{
        return format!("{:.2}K",s1);
    }else{
        let s1=s1/1024f64;
        return format!("{:.2}M",s1);

    }
}