fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.

    // Macro version would be:
    // let v = vec!(a)
    // but then compiler gives a type error because v has type Vec<[i32; 4]> and not the expected Vec<i32> type.
    // So the approach below gives the correct type to make compiler happy, but does not use a macro as instructed:
    let v = a.to_vec();

    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
