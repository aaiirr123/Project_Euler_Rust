pub fn largest_palindrome_product(num_digits: u32) -> i64
{
    // if one digit 9 = (10 to pow of 1) - 1
    // if two digits 99 = (10 to pow of 2) - 1
    let max_factor : i64 = 10_i64.pow(num_digits);
    let mut max : i64 = 0;

    // now we need to go through every possible multiplication
    for i in (1..max_factor).rev()
    {
        if i * (max_factor - 1) < max
        {
            return max;
        }

        for j in (1..max_factor).rev()
        {
            let sum =  i * j;
    
            if is_palindrome(sum as u64)
            {
               if sum > max
               {
                   max = sum;
               }
            }
        }
    }

    max_factor
}

pub fn is_palindrome(val : u64) -> bool
{
    let mut nums : Vec<u8> = Vec::new(); 
    let mut val_copy = val;

    while val_copy > 0
    {
        let least_sig = val_copy % 10;
        val_copy /= 10;
        nums.push(least_sig as u8)
    }
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r
    {
        if nums[l] != nums[r]
        {
            return false
        }

        l += 1;
        r -= 1;
    }

    true
    
}
