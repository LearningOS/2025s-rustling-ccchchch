fn sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }
        // 如果这一轮没有发生交换，说明数组已经有序
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut numbers = vec![4, 2, 7, 1, 3];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 4, 7]);

        let mut strings = vec!["banana", "apple", "cherry", "date"];
        sort(&mut strings);
        assert_eq!(strings, vec!["apple", "banana", "cherry", "date"]);
    }
}