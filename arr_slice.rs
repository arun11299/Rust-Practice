use std::mem;

fn analyze_slice(slice : &[i32])
{
    println!("Length of array is: {}", slice.len());
    println!("First element: {}", slice[0]);
    println!("Last element: {}", slice[slice.len() - 1]);
}

fn main()
{
    let mut arr : [i32; 5] = [1, 2, 3, 4, 5];
    analyze_slice(&arr);
    analyze_slice(&arr[1..4]);

    println!("Array occupies {} bytes", mem::size_of_val(&arr));

    arr = [40; 5];
    analyze_slice(&arr);
}
