fn main() {
    let num1 = [1,2];
    let num2 = [4,4];
    let mut num12: Vec<i32>= Vec::new();
    let mut median: i32= 0;
    for i in 0..num1.len(){num12.push(num1[i]);}
    for i in 0..num2.len(){num12.push(num2[i]);}
    num12.sort();
    println!("{:?}",num12);
    if num12.len() % 2 == 1{
        median = num12[((num12.len() + 1)/2)-1];
        println!("The median is {}",median);
    }else{
        let median = (((num12[((num12.len()/2))-1]) + num12[(((num12.len()/2) + 1))-1])) as f32 /2.0  ;
        println!("The median is e {}",median);
    }
}