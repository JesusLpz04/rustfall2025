
//Temp converter
fn assignment_1(){
    let freeze:f64=32.0;
    fn fahrenheit_to_celsius(x:f64) -> f64{
        let n=x;
        let n= n-32.0;
        let n = n*5.0/9.0;
        return n;
    }
    fn celsius_to_fahrenheit(x:f64) -> f64{
        let m= x;
        let m= m*9.0/5.0;
        let m= m+32.0;
        return m;
    }
    celsius_to_fahrenheit(52.0);

    for i in 0..5{
        println!("{}",fahrenheit_to_celsius(freeze+i as f64))
    }
}
fn assignment_2(){
    fn is_even(x:i32) ->bool{
        if x%2==0{
                return true;
            }
            else{
                return false;
            }
        }
    
    let numbers: [i32; 10] = [11, 21, 23, 4, 5,16,7,10,9,15];
    let mut n:i32 =0;
    let mut sum:i32 =0;
        for x in numbers.iter(){
            sum=sum+*x;
            if *x>n{
                n=*x;
            }
            if x%5==0 && x%3==0 {
                println!("fizzbuzz")
            }
            else if x%5==0{
                println!("buzz")
            }
            else if x%3==0{
                println!("fizz")
            }
            else{
                if is_even(*x)==true{
                    println!("even")
                }
                else{
                    println!("odd")
                }

            }
        }
        println!("{}",sum);
        println!("{}",n);
}
fn assignment_3(){
    let secret:i32 = 47;
    let mut guess:i32=40;
    fn check_guess(guess: i32, secret: i32) -> i32{
        if guess == secret{
            return 0;
        }
        else if guess> secret{
            return 1;
        }
        else{
            return -1;
        }
    }
    let mut count:i32 = 0;
    while guess!= secret{
        count = count +1;
        let gu:i32=check_guess(guess,secret);
        if gu==0{
            println!{"correct"}
            break;
        }
        else if gu>0{
            println!{"guess too high"}
        }
        else{
            println!{"guess too low"}
        }
        guess= guess+1;
    }
    println!("total guesses: {}",count);

}
fn main() {
    assignment_1();
    assignment_2();
    assignment_3();
}