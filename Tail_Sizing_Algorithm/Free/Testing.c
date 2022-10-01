#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(int argc, char* argv[]) {

//Desired Static Margin of the aircraft
double s_m = 0.2;
//Chord length of the main wing
double c_hr_a = 0.4; 
//Aerodynamic Center of main wing
double x_ac_a = 0.1; 
//Aerodynamic Center of wing B (tail wing)
double x_ac_b = 1.9; 
//The total mass of the front fuselage
double m_f = 1.2; 
//Center of Gravity of front fuselage
double x_cg_f = 0.01; 
//Mass of the Tail Boom
double m_tb = 0.2; 
//Location of Center of Gravity of Tail Boom
double x_cg_tb = 0.3; 
//Mass of the other things on the tail
double m_tm = 0.0; 
//Location of the center of gravity of the other things on tail
double x_cg_tm = 0.0; 
//Center of Gravity of wing B (tail wing)
double x_cg_wb = 0.9; 
//Derivative of Lift coefficient with respect to Angle of attack for wing B (Tail Wing)
double c_L_alpha_b = 0.1; 
//Derivative of Lift coefficient with respect to Angle of attack for wing A (Main Wing)
double c_L_alpha_a = 0.2; 
//Fixed Constant for relating Area to Mass of wing B
double c_bf = 0.1; 
//Fixed Constant for relating Area to Mass of wing B
double c_bs = 0.2; 
//Area of wing A (Main Wing)
double A_w_a = 2.0; 

//Computation of all intermediate k-constants
double k_1_a = s_m*c_hr_a - x_ac_a;
double k_1_b = x_ac_b -s_m*c_hr_a; 
double k_2_u = m_f*x_cg_f + m_tb*x_cg_tb + m_tm*x_cg_tm; 
double k_2_l = m_f + m_tb + m_tm; 
double k_3_lc = k_1_a*k_2_l + k_2_u; 
double k_3_ld = k_1_a + x_cg_wb; 
double k_3_rc = k_1_b*k_2_l - k_2_u; 
double k_3_rd = k_1_b - x_cg_wb; 
double k_4_a = c_L_alpha_b*k_3_rd*c_bs; 
double k_4_b = c_L_alpha_b*(k_3_rc+k_3_rd*c_bf)-A_w_a*c_L_alpha_a*k_3_ld*c_bs; 
double k_4_c =  - A_w_a*c_L_alpha_a*(k_3_lc+k_3_ld*c_bf); 

//Computation of the Tail Wing Area
double A_wb = (-k_4_b    +     pow(k_4_b*k_4_b-4.0*k_4_a*k_4_c,0.5))/(2.0*k_4_a);

printf("A_wb = %lf\n", A_wb);



return EXIT_SUCCESS;}
