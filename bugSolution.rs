fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    let mut values = Vec::with_capacity(len);
    unsafe {
        std::ptr::copy_nonoverlapping(v.as_ptr(), values.as_mut_ptr(), len);
    }

    values[0] = 4; // Modify the copy

    println!("Original vector: {:?}", v); // Original vector remains unchanged
    println!("Modified copy: {:?}", values); // Modified copy is printed
} 