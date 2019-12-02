fn main() {
    check_leap_year(2000);
    check_leap_year(1900);
    check_leap_year(1832);
    check_leap_year(1921);
    check_leap_year(104);
    check_leap_year(2400);
}

fn check_leap_year(year: i32){
    println!("{}: {}",year , is_leap_year(year));
}

fn is_leap_year(year: i32) -> bool{
    return year % 4 == 0 && year % 100 != 0 || is_divisible_by_400(year);
}

fn is_divisible_by_400(year: i32) -> bool {
    return year % 400 == 0;
}