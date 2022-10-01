fn main() {
//Desired Static Margin of the aircraft
let s_m = 0.2;
//Chord length of the main wing
let c_hr_a:f64 = 0.4; 
//Aerodynamic Center of main wing
let x_ac_a:f64 = 0.1; 
//Aerodynamic Center of wing B (tail wing)
let x_ac_b:f64 = 1.9; 
//The total mass of the front fuselage
let m_f:f64 = 1.2; 
//Center of Gravity of front fuselage
let x_cg_f:f64 = 0.01; 
//Mass of the Tail Boom
let m_tb:f64 = 0.2; 
//Location of Center of Gravity of Tail Boom
let x_cg_tb:f64 = 0.3; 
//Mass of the other things on the tail
let m_tm:f64 = 0.0; 
//Location of the center of gravity of the other things on tail
let x_cg_tm:f64 = 0.0; 
//Center of Gravity of wing B (tail wing)
let x_cg_wb:f64 = 0.9; 
//Derivative of Lift coefficient with respect to Angle of attack for wing B (Tail Wing)
let c_L_alpha_b:f64 = 0.1; 
//Derivative of Lift coefficient with respect to Angle of attack for wing A (Main Wing)
let c_L_alpha_a:f64 = 0.2; 
//Fixed Constant for relating Area to Mass of wing B
let c_bf:f64 = 0.1; 
//Fixed Constant for relating Area to Mass of wing B
let c_bs:f64 = 0.2; 
//Area of wing A (Main Wing)
let A_w_a:f64 = 2.0; 

//Computation of all intermediate k-constants
let k_1_a:f64 = s_m*c_hr_a - x_ac_a;
let k_1_b:f64 = x_ac_b -s_m*c_hr_a; 
let k_2_u:f64 = m_f*x_cg_f + m_tb*x_cg_tb + m_tm*x_cg_tm; 
let k_2_l:f64 = m_f + m_tb + m_tm; 
let k_3_lc:f64 = k_1_a*k_2_l + k_2_u; 
let k_3_ld:f64 = k_1_a + x_cg_wb; 
let k_3_rc:f64 = k_1_b*k_2_l - k_2_u; 
let k_3_rd:f64 = k_1_b - x_cg_wb; 
let k_4_a:f64 = c_L_alpha_b*k_3_rd*c_bs; 
let k_4_b:f64 = c_L_alpha_b*(k_3_rc+k_3_rd*c_bf)-A_w_a*c_L_alpha_a*k_3_ld*c_bs; 
let k_4_c:f64 =  - A_w_a*c_L_alpha_a*(k_3_lc+k_3_ld*c_bf); 

//Computation of the Tail Wing Area
let A_wb:f64 = (-k_4_b + (k_4_b.powf(2.0)-4.0*k_4_a*k_4_c).powf(0.5))/(2.0*k_4_a);

println!("A_wb = {}", A_wb);

}
