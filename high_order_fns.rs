
fn is_odd(num : u32) -> bool
{
    num % 2 == 1
}

fn iterative_accum() -> u32
{
    let mut acc : u32 = 0;
    let upper_limit = 1000;

    for n in 0.. {
        let n_sq = n * n;
        if n_sq > upper_limit {
            break
        }
        if is_odd(n_sq) {
            acc += n_sq;
        }
    }

    return acc;
}

fn cool_accum() -> u32
{
    let upper_limit = 1000;

    let acc = (0..).map(|n| n * n)
                   .take_while(|&n_sq| n_sq <= upper_limit)
                   .filter(|&num| is_odd(num))
                   .fold(0, |acc, n_sq| acc + n_sq)
                   ;
    return acc;
}

fn main() {
    let res1 = iterative_accum();
    println!("Iterative accum result: {}", res1);

    let res2 = cool_accum();
    println!("Cool accum result: {}", res2);

}
