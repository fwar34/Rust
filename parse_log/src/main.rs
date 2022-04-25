// #[macro_use]
// extern crate lazy_static;
// use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;

// String，＆str，Vec <u8>和＆[u8]的惯用转换
// &str    -> String--| String::from(s) or s.to_string() or s.to_owned()
// &str    -> &[u8]---| s.as_bytes()
// &str    -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
// String  -> &str----| &s if possible* else s.as_str()
// String  -> &[u8]---| s.as_bytes()
// String  -> Vec<u8>-| s.into_bytes()
// &[u8]   -> &str----| s.to_vec() or s.to_owned()
// &[u8]   -> String--| std::str::from_utf8(s).unwrap(), but don't**
// &[u8]   -> Vec<u8>-| String::from_utf8(s).unwrap(), but don't**
// Vec<u8> -> &str----| &s if possible* else s.as_slice()
// Vec<u8> -> String--| std::str::from_utf8(&s).unwrap(), but don't**
// Vec<u8> -> &[u8]---| String::from_utf8(s).unwrap(), but don't**

#[allow(dead_code)]
fn test_file1() {
    let path = Path::new("test.txt");
    let display = path.display();

    if let Ok(mut file) = File::open(path) {
        println!("file {:?}", file);
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("cloudn't read {}: {}", display, why),
            Ok(_) => println!("{} contains:\n{}", display, s),
        }
    } else {
        panic!("cloudn't open {}", display)
    }
}

#[allow(dead_code)]
fn test_file2() {
    let path = Path::new("test.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("cloudn't open {}: {}", display, why),
        Ok(file) => file,
    };
    println!("file {:?}", file);

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("cloudn't read{}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}

fn test_file3() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

#[allow(dead_code)]
fn test_file4() -> std::io::Result<()> {
    let file = File::create("test.txt").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(b"sdlkfsldkfj xxx")?;
    writer.flush()?;
    Ok(())
}

#[allow(dead_code)]
fn test_regex1() {
    let s = "123-4567-89,987-6543-21";
    let r = Regex::new(r"\d{3}-(\d{4}).*").unwrap();
    if r.is_match(s) {
        println!("found!");
    }

    for cap in r.captures_iter(s) {
        println!("found group {}", &cap[1]);
    }
}

#[allow(dead_code)]
fn guess() {
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("Please type a number:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: u32 = guess.trim().parse().expect("Please type a num!");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

const LOG: &str = "/home/feng/bug/1157238220.log.1";
const USERID: &str = "/home/feng/bug/userid.txt";
const RET: &str = "/home/feng/bug/ret.txt";

fn find_exit(
    userid: &String,
    log: &mut BufReader<File>,
    ret: &mut BufWriter<File>,
) -> io::Result<bool> {
    let re: Regex = Regex::new(
        &(r"(\d{2}:\d{2}:\d{2}).*BMS user exit conference success.*".to_string() + &userid),
    )
    .unwrap();

    log.seek(SeekFrom::Start(0))?;

    for line in log.lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            if cap.len() != 0 {
                // println!("{:?}", cap);
                let data = format!("{} {}\n", userid, &cap[1]);
                ret.write_all(data.as_bytes())?;
                return Ok(true);
            }
        }
    }
    return Err(Error::new(ErrorKind::NotFound, "Not found!"));
}

fn find_offline(userid: &String, log: &mut BufReader<File>) -> io::Result<String> {
    let mut ret = String::new();
    let re: Regex = Regex::new(
        &(r"(?P<time>\d{2}:\d{2}:\d{2}).*userOffline_handle.*".to_string()
            + &userid
            + ".*serviceType=0x1,.*channelType=50c"),
    )
    .unwrap();
    log.seek(SeekFrom::Start(0))?;

    for line in log.lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            if cap.len() != 0 {
                ret = cap[1].to_string();
                break;
            }
        }
    }

    if !ret.is_empty() {
        return Err(Error::new(ErrorKind::NotFound, "Not found!"));
    }

    return Ok(ret);
}

fn find_online(userid: &String, log: &mut BufReader<File>) -> io::Result<String> {
    let mut ret = String::new();
    let re: Regex = Regex::new(
        &(r"(?P<time>\d{2}:\d{2}:\d{2}).*userOnline_handle.*".to_string()
            + &userid
            + ".*serviceType=0x1,.*channelType=50c"),
    )
    .unwrap();

    log.seek(SeekFrom::Start(0))?;

    for line in log.lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            if cap.len() != 0 {
                ret = cap[1].to_string();
            }
        }
    }

    if !ret.is_empty() {
        Ok(ret)
    } else {
        Err(Error::new(ErrorKind::NotFound, "Not found!"))
    }
}

fn parse() -> io::Result<bool> {
    let file_userid = File::open(USERID).unwrap();
    let reader_userid = BufReader::new(file_userid);
    let file_log = File::open(LOG).unwrap();
    let mut reader_log = BufReader::new(file_log);
    let file_ret = File::create(RET).unwrap();
    let mut writer_ret = BufWriter::new(file_ret);

    for (index, line) in reader_userid.lines().enumerate() {
        let line = line.unwrap();
        println!("{}:{}", index + 1, line);
        if let Err(_) = find_exit(&line, &mut reader_log, &mut writer_ret) {
            // 找不到退会日志，继续查找下线
            if let Ok(offline) = find_offline(&line, &mut reader_log) {
                // 找到了 offline 则继续查找 online
                if let Ok(online) = find_online(&line, &mut reader_log) {
                    if offline >= online {
                        writer_ret.write_all(format!("{} {}", line, offline).as_bytes())?;
                    } else {
                        writer_ret.write_all(format!("{} 11:44:35", line).as_bytes())?;
                    }
                } else {
                    println!("Not found {} online", line);
                }
            } else {
                writer_ret.write_all(format!("{} 11:44:35", line).as_bytes())?;
            }
        }
    }
    writer_ret.flush()?;
    return Ok(true);
}

#[allow(dead_code)]
fn test_lifetime_generic<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    test_regex1();
    // test_file1();
    // test_file2();
    test_file3();
    let _ = test_file4();
    // guess();
    let _ = parse();
    // test_lifetime_generic("str", "sss", String::from("xxxxx"));
}
