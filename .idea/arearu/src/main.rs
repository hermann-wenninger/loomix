fn parsenum(s:Vec<String>)-> String{
    let mut sammler:i32 = 0;
    for k in s{
        sammler += match to_int(&k){
            Some(v)=>v,
            None=>0,
        };
    }
    sammler.to_string()
}

fn to_int(s:&String)->Option<i32>{
    s.parse::<i32>().ok()
}

fn main() {
    let numvec = vec!["10".to_string(),"20".to_string(),"30".to_string(),"abc".to_string()];
    let result = parsenum(numvec);
    println!("The sum is: {}", result);
}
