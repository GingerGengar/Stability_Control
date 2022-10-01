
pub struct Collection {
    var1: f64,
    var2:f64,
}


    


impl Collection {
fn default() -> Collection {Collection{var1: 1.0, var2: 2.0}}
}

fn main(){
let stuff = Collection{var2: 6.0, ..Collection::default()};
println!("var1: {}, var2: {}", stuff.var1, stuff.var2);

}

