pub fn sort(input: Vec<i32>) -> Vec<i32> {
	if input.is_empty() {
		return Vec::new();
	}

	if input.len() == 1{
		return input;
	}
	do_sort(input)
}

fn do_sort(mut input: Vec<i32>) -> Vec<i32> {
	let lenght = input.len() - 1;
	let input = input.as_mut_slice();
	recursive_sort(input, 0, lenght);

	let mut result = Vec::new();
	for element in input {
		result.push(*element)
	}
	result
}

fn recursive_sort(input: &mut[i32], left:usize, right:usize) -> () {
	let index = partition(input, left, right);
	if left < index -1 {
		recursive_sort(input, left, index -1);
	}
	if index < right {
		recursive_sort(input, index, right);
	}
}

fn partition(input: &mut[i32], left:usize, right:usize) -> usize {
	let mut i = left;
	let mut j = right;
	let pivot = input[divide_point(left,right)];

	while i<=j {
		while input[i] < pivot {
			i = i + 1;
		}
		while input[j] > pivot {
			j = j - 1;
		}
		if i <= j {
			replace_one(input, i, j);
			i = i + 1;
			j = j - 1;
		}
	};
	i
}

fn divide_point(l:usize, r:usize) -> usize{
	(l+r) / 2
}

fn replace_one(input: &mut[i32], i1:usize, i2:usize) -> (){
	let aux = input[i1];
	input[i1] = input[i2];
	input[i2] = aux;
}

#[test]
fn should_sort_vec_empty() {
	assert_eq!(sort(Vec::new()), Vec::new());
}

#[test]
fn should_sort_one() {
	//given
	let input:Vec<i32> = vec![2];
	let expected:Vec<i32> = vec![2];

	//when
	let result = sort(input);

	//then
	assert_eq!(result, expected);
}

#[test]
fn should_sort_two() {
	//given
	let input:Vec<i32> = vec![2,1];
	let expected:Vec<i32> = vec![1,2];

	//when
	let result = sort(input);

	//then
	assert_eq!(result, expected);
}

#[test]
fn should_sort_vec() {
	//given
	let input:Vec<i32> = vec![2, 3, 4, 5, 1];
	let expected:Vec<i32> = vec![1, 2, 3, 4, 5];

	//when
	let result = sort(input);

	//then
	assert_eq!(result, expected);
}

#[test]
fn should_sort() {
	//given
	let input: &mut[i32] = &mut [2, 3, 4, 5, 1];
	let expected: [i32; 5] = [1, 2, 3, 4, 5];

	//when
	recursive_sort(input, 0, 4);

	//then
	assert_eq!(input, expected);
}

#[test]
fn should_divide_array() {
	//given
	let input: &mut[i32] = &mut [2, 3, 4, 5, 1];
	let expected: [i32; 5] = [2, 3, 1, 5, 4];

	//when
	partition(input, 0, 4);

	//then
	assert_eq!(input,expected);
}

#[test]
fn should_select_divide_point() {
	//when
	let result = divide_point(2,2);

	//then
	assert_eq!(result,2);
}

#[test]
fn should_replace_elements() {
	//given
	let input: &mut[i32] = &mut[1, 2, 3, 4, 5];
	let expected: [i32; 5] = [1, 3, 2, 4, 5];

	//when
	replace_one(input,1,2);

	//then
	assert_eq!(input,expected);
}