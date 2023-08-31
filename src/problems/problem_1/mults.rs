// Concise version problem 1
pub fn sum_of_multiples_c(upper_limit: i32) -> i32
{
    //let smallest = multiples.iter().min();
    (1..upper_limit).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

// Readable version problem 1
pub fn sum_of_multiples(upper_limit: i32, multiples: Vec<i32>) -> i32
{
    let mut sum = 0;


    for i in 0..upper_limit 
    {
        for multiple in multiples.iter()
        {
            if is_multiple(i, *multiple)
            {
                sum += i;
                break;
            }
        }
    }

    sum
}

fn is_multiple(value: i32, possible_multiple: i32) -> bool
{
    if value % possible_multiple == 0 
    {
        return true;
    }
    else 
    {
        return false;
    }
}
