extern crate sorting;

fn main() {
	let data = vec![5,-1,23,1111,0,1];
	println!("Sorted numbers {:?}", sorting::sorting::quicksort::sort(data));
}
