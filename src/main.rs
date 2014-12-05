use std::io;

enum Sign {
  Zero,
  Negative,
  Positive,
}

fn findSign(a: int) -> Sign {
  if a < 0 { Sign::Negative }
  else if a > 0 { Sign::Positive }
  else { Sign::Zero }
}

fn main() {
  loop{
    println!("Enter a number to determine the sign: ");
    let input = io::stdin().read_line().ok().expect("Failed to read line.");
    let input_num: Option<int> = from_str(input.as_slice().trim());
    let num = match input_num{
      Some(num) => num,
      None  => break
    };
    match findSign(num){
      Sign::Negative => {
        println!("The number {} is negative", num);
        },
        Sign::Positive => {
          println!("The number {} is positive", num);
          },
        Sign::Zero => {
          println!("The number {} is neither positive nor negative", num);
        },
    }

  }

}
