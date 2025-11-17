fn main() {
 let mut a: [i8; 5] = [5, 4, 3, 2, 1];
a[2] = 0;
println!("a: {a:?}");
fn get_index() -> usize {
4
}
let index = get_index();
println!("Index: {index}");
println!("Element at index {}: {}", index, a[index]);
let xyz:(i8,i8 bool,i32,String) = (1,2,true,45,"hello".to_string());
println!("Tuple: {:?}", xyz);
let (x,y,z,b,s) = xyz;
println!("Destructured: x={}, y={}, z={}, b={}, s={}", x,y,z,b,s);
}
