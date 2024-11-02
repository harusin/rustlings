fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = match a {
            [_, 2, 3, 4, _] => [2, 3, 4],
            _ => [0; 3]
        };

        assert_eq!([2, 3, 4], nice_slice);
    }
}
