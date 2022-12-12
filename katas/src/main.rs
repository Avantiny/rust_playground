//

fn multiply(a:i32, b:i32) -> i32 {
    a * b
  }

// Find Maximum and Minimum Values of a List

fn minimum(arr: &[i32]) -> i32 {
    return *arr.iter().min().unwrap();
}

fn maximum(arr: &[i32]) -> i32 {
    return *arr.iter().max().unwrap();
}

fn main() {
    let arr = [1,2,3];
    println!("{} {}", minimum(&arr), maximum(&arr));
    println!("{}", multiply(6, 9))
}
