/*List of Global Constants*/
pub const g:f64 = 9.81; //Gravitational constant (m/s^2)
pub const rho:f64 = 1.225; //Density of air (kg/m^3)

/*Given dynamic pressure, compute velocity*/
pub fn q2vel(q:f64)->f64 {((2.0*q)/rho).powf(0.5)}

/*Given Lift coeff and Dynamic pressure, and area, compute lift force*/
pub fn cL2L(c_L:f64, q:f64, A:f64)->f64 {q*A*c_L}
