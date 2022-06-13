use std::io;

use rand::Rng;

fn main() {
    // let array_root: [i32; 10] = [12, 3, 6, 23, 78, 23, 5, 12, 4, 1];
    // let array_test: [i32; 3] = [1, 4, 5];

    // println!("Root array: {:?}", array_root);
    // println!("Test array: {:?}", array_test);
    // let mut tmp_index_01 = 0;

    // let mut str_root = "".to_string();/=u80

    // let name: String = String::from("hello");

    // let str: &str = &name[0..4];

    // print!("name= {}, str = {}", name, str);

    // let a: i32 = 5;

    // let str: &str = match a {
    //     6 => "a",
    //     _ => " khong phai la 5",
    // };
    // println!("aa {}", str);

    // let mut name = String::from("helo");
    // let str = &name[0..4];

    // println!("hello: {}", name);

    // name.push_str("abvc");
    // println!("hello: {}", name);

    // let input = "Hello world";

    // let mut output = String::new();

    // for ch in input.chars() {
    //     output.insert(0, ch);
    // }

    // println!("aaa= {}", input.chars().rev().collect::<String>());
    // loop {
    //     let mut line = String::new();
    //     println!("enter a number: ");
    //     io::stdin().read_line(&mut line).unwrap();
    //     // line.pop(); // \n
    //     println!("input = {} ", line);

    //     let number = line.parse::<i32>().unwrap();

    //     if number < 0 {
    //         break;
    //     }

    //     let rand_number = rand::thread_rng().gen_range(0..100);

    //     if number == rand_number {
    //         println!("trung nhau");
    //     } else {
    //         println!("no");
    //     }
    // }

    // println!("end");

    let org_arr: [i32; 10] = [12, 3, 6, 23, 78, 23, 10, 12, 4, 1];
    let sub_arr: [i32; 3] = [1, 4, 5];

    let mut tmp = 0;

    let mut str_org = "".to_string();
    let mut str_sub = "".to_string();
    while tmp < sub_arr.len() {
        str_sub.push_str(&sub_arr[tmp].to_string());
        tmp += 1;
    }
    tmp = 0;
    while tmp < org_arr.len() {
        str_org.push_str(&org_arr[tmp].to_string());
        tmp += 1;
    }
    if str_org.contains(&str_sub) {
        println!("Array sub thuoc Array org")
    } else {
        println!("Array sub khong thuoc Array org")
    }
}
