use std::io;
fn main(){
    println!("enter a number");
    let mut start=String::new();
    io::stdin().read_line(&mut start).expect("fail");
    
    let start: u32=match start.trim().parse(){
        Ok(num)=>num,
        Err(_)=>0,
    };
    println!("enter 2nd number");
    let mut end=String::new();
    io::stdin().read_line(&mut end).expect("Fail");
    
    let end: u32=match end.trim().parse(){
        Ok(num)=>num,
        Err(_)=>0,
    };
    
    
    println!("prime num btw {} and {}",start,end);
    for num in start..=end{
        if is_prime(num){
            println!("{}",num);
        }
    }
}

fn is_prime(num:u32)->bool {
    if num<=1{
        return false;
    }
    for i in 2..=(num/2){
        if num%i==0{
            return false;
        }
    }
    true
}
