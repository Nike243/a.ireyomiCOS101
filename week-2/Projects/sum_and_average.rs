fn main(){
let items=["Toshiba","Mac","HP","Dell","Acer"];
let sales=[450000.00,1500000.00,750000.00,2850000.00,250000.00];
let sum:f64 = sales.iter().sum();
let average = sum /sales.len() as f64;
println!("{:?}",items );
println!("Sum of sales = {:.2}", sum);
println!("Average sales = {:.2}", average);

}