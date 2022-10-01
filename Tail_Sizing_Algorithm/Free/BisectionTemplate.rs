/*If the two numbers have different signs, return true, if not, return false*/
fn DiffSign(num1:f64, num2:f64) -> bool {let prod:f64 = num1*num2; 
    if prod<0.0 {true} else {false}}

/*Function x*/
fn y(x: f64) -> f64 {x - 0.3424}


/*All Data related to the bisection solver*/
struct Bisection_Solver {
    Tolerance:f64,
    guess1:f64, //Lower bound guess of independent variable
    guess2:f64, //Upper bound guess of independent variable
    guess3:f64, //Middle guess of independent variable
    yval1:f64, //lower bound guess of dependent variable
    yval2:f64, //Upper bound guess of dependent variable
    yval3:f64} //Middle guess of depdenent variable

/*All the functions specific to the Bisection Solver */
impl Bisection_Solver {
/*Update Bisection*/
fn BisectionUpdate(&mut self) {self.guess3 = (self.guess1+self.guess2)/2.; 
    self.yval3 = y(self.guess3);}

/*Bisection Solver*/
fn BisectionMethod(&mut self){
    //From initial x value, find the initial bisection
    self.guess3 = (self.guess1+self.guess2)/2.0;
    //From bisection in x, find the corresponding y at the bisection
    self.yval1= y(self.guess1); self.yval2 = y(self.guess2); self.yval3 = y(self.guess3);
    while (self.yval1).abs()>self.Tolerance {self.BisectionUpdate(); 
    if DiffSign(self.yval1, self.yval3) {self.guess2 = self.guess3; self.yval2 = self.yval3;}
    else if DiffSign(self.yval2, self.yval3) {self.guess1 = self.guess3; self.yval1=self.yval3;}
    else {println!("Bisection method fails"); break;}}}
}


fn main(){
let mut solve = Bisection_Solver{
Tolerance : 1e-6,
guess1 : 0., //Lower bound guess of independent variable
guess2 : 1., //Upper bound guess of independent variable
guess3 : 0., //Middle guess of independent variable
yval1 : 0., //lower bound guess of dependent variable
yval2 : 0., //Upper bound guess of dependent variable
yval3 : 0. //Middle guess of depdenent variable
};
solve.BisectionMethod();
println!("Guess 1: {}, Guess 2: {}", solve.guess1, solve.guess2);
}
