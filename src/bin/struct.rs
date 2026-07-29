struct Don{
    name:String,
    age:i32,
    roll:u32,
    active:bool,
}



fn main() {


let lee=Don{
    name:String::from("jee"),
    age:21,
    roll:12311,
    active:true,
};
let lee2=Don{
    name:String::from("je1221e"),
    age:22,
    roll:123121,
    active:false,
};

let lee3=Don{
    name:String::from("kimjoo"),
    age:11,
    roll:1231,
    active:true,
};



print!("{}",lee2.name);
print!("{}",lee2.age);
print!("{}",lee2.roll);
print!("{}",lee2.active);


print!("{}",lee3.name);
print!("{}",lee3.age);
print!("{}",lee3.roll);
print!("{}",lee3.active);



print!("{}",lee.name);
print!("{}",lee.age);
print!("{}",lee.roll);
print!("{}",lee.active);

    println!("Hello, world!");
}
