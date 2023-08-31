
// Readable version
pub fn sum_even_fib(limit: i64) -> i64
{
    let mut l = 1;
    let mut r = 2;
    let mut curr = 3;
    let mut sum = 2;

    while curr < limit
    {
        curr = l + r;
        l = r;
        r = curr;
        if curr % 2 == 0
        {
            sum += curr;
        }
    }   

    sum



}