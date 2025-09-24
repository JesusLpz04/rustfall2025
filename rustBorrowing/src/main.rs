fn problem1(){
    fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
    }


    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result);

}

fn problem2(){
    fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut clone = s.clone();
    clone.push_str("World!");
    clone
    }


    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

}
fn problem3(){
    #[allow(unused_variables, unused_mut)]
    fn sum(total: &mut i32, low: i32, high: i32) {
        // Write your code here!
        for i in low..high+1 {
        *total += i;
    }
    }

    // create necessary variables and test your function for low 0 high 100
    // total should be 5050
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("The sum from 0 to 100 is: {}", total);
}
fn main() {
    problem1();
    problem2();
    problem3();
}
