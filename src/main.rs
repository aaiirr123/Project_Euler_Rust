mod problems {
    pub mod problem_1 {
        pub mod mults;
    }

    pub mod problem_2 {
        pub mod even_fib;
    }

    pub mod problem_3 {
        pub mod largest_prime;
    }

    pub mod problem_4 {
        pub mod largest_pal_prod;
    }
}
// need to use a different method for importing
use crate::problems::problem_1::mults;
use crate::problems::problem_2::even_fib;
use crate::problems::problem_3::largest_prime;
use crate::problems::problem_4::largest_pal_prod;


fn main()
{
    println!("{} is the largest palindrome made up of two
             three digit numbers",
             largest_pal_prod::largest_palindrome_product(3)
             );
}


#[test]
fn test_sum_of_multiples()
{
    let test_vec = vec![3,5];
    assert_eq!(mults::sum_of_multiples(10, test_vec), 23);
}

#[test]
fn test_largest_prime()
{
    assert_eq!(largest_prime::is_prime(35), false);
    assert_eq!(largest_prime::is_prime(7), true);
    assert_eq!(largest_prime::largest_prime(13195),29);
   // assert_eq!(largest_prime::largest_prime(600851475143),29);
}

#[test]
fn test_even_fib()
{
    assert_eq!(even_fib::sum_even_fib(3), 2);
    assert_eq!(even_fib::sum_even_fib(10), 10);

}

#[test]
fn test_largest_pal_prod()
{

    assert_eq!(largest_pal_prod::is_palindrome(99), true);
    assert_eq!(largest_pal_prod::is_palindrome(923213), false);
    assert_eq!(largest_pal_prod::is_palindrome(991199), true);
    assert_eq!(largest_pal_prod::largest_palindrome_product(2), 9009)
}
