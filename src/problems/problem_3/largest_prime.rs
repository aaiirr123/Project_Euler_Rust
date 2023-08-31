
// readable
pub fn largest_prime(n: i64) -> i64 
{
    // the starting point for a prime number
    // would have to be at the half way mark
    // we also would only have to check even numbers
    //
    for i in (1..(n as f64).sqrt() as i64).rev()
    {
        if n % i == 0
        {
            if is_prime(i)
            {
                return i;
            }
        }
    }

    return 0;
}
pub fn is_prime(n: i64) -> bool
{
    if n <= 1 { return false };
    for i in 2..(((n as f64).sqrt() as i64) + 1) 
    {
        if n % i == 0
        {
            return false
        }
    }

    return true;
}
// clever
