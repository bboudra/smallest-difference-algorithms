/// Will take n^2 operations, worst case time O(n^2)
fn brute_force_smallest_difference(numbers_to_compare: Vec<i32>) -> [i32; 2]{
    let mut current_smallest_value: i32;
    let mut first_close_value: i32;
    let mut second_close_value: i32;
    current_smallest_value = (numbers_to_compare[0]- numbers_to_compare[1]).abs();
    first_close_value = numbers_to_compare[0];
    second_close_value = numbers_to_compare[1];
    for first_number in 0..numbers_to_compare.len(){ //runs n times
        for second_number in (first_number + 1)..numbers_to_compare.len(){ //runs n^2 times
            let first_value = numbers_to_compare[first_number];
            let second_value = numbers_to_compare[second_number];
            let difference: i32 = (first_value - second_value).abs();
            if difference < current_smallest_value {
                current_smallest_value = difference;
                first_close_value = first_value;
                second_close_value = second_value;
            }
        }
    }
    let closest_values = [first_close_value, second_close_value];
    closest_values
}

/// Will take (n log) + n operations, worst case time O(n log n)
fn transform_and_conquer_smallest_difference(mut numbers_to_compare: Vec<i32>) -> [i32; 2] {
    numbers_to_compare.sort();  // Sort runs in n log(n) according to the documentation
    let sorted_numbers_to_compare: Vec<i32> = numbers_to_compare;
    let mut smallest_difference = sorted_numbers_to_compare[1] - sorted_numbers_to_compare[0];
    let mut first_close_value= sorted_numbers_to_compare[0];
    let mut second_close_value = sorted_numbers_to_compare[1];

    for current_iteration in 0..(sorted_numbers_to_compare.len()-1) { //Takes n operations
        let first_value = sorted_numbers_to_compare[current_iteration];
        let second_value = sorted_numbers_to_compare[current_iteration + 1];
        let difference: i32 = (first_value - second_value).abs();
        if difference < smallest_difference {
            smallest_difference = difference;
            first_close_value = first_value;
            second_close_value = second_value;
        }
    }
        [first_close_value, second_close_value]
}

#[cfg(test)]
mod test {
    use super::brute_force_smallest_difference;
    use super::transform_and_conquer_smallest_difference;
    #[test]
    fn should_return_smallest_difference_is_2_3(){
        //given

        //when
        let result: [i32; 2] = brute_force_smallest_difference( vec![0,2,3,8,11] );
        //then
        assert_eq!([2,3], result)
    }

    #[test]
    fn should_return_the_only_two_numbers(){
        //given

        //when
        let result: [i32; 2]= brute_force_smallest_difference( vec![0,25] );
        //then
        assert_eq!([0,25], result)
    }

    #[test]
    fn should_return_first_set_of_closest_numbers(){
        //given

        //when
        let result: [i32; 2] = brute_force_smallest_difference( vec![0,1,2] );
        //then
        assert_eq!([0,1], result)
    }

    #[test]
    fn should_state_2_3_are_closest_values(){
        //given

        //when
        let result: [i32; 2] = transform_and_conquer_smallest_difference( vec![0, 2, 3, 8, 12] );

        //then
        assert_eq!([2,3], result);
    }

    #[test]
    fn should_state_only_two_numbers_are_closest() {
        //given

        //when
        let result: [i32; 2] = transform_and_conquer_smallest_difference( vec![0, 25] );

        //then
        assert_eq!([0, 25], result)
    }

    #[test]
    fn should_state_first_set_of_closest_values_are_closest_values() {
        //given

        //when
        let result: [i32; 2] = transform_and_conquer_smallest_difference( vec![0, 1, 2, 3]);

        //then
        assert_eq!([0, 1], result)
    }
}
