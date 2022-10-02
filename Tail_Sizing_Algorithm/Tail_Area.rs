use g; use q2vel; use cL2L; use Sizing_Tail_Area;
impl Sizing_Tail_Area{

/*Sizes the area of wing B based on location of neutral point and center of gravity*/
pub fn Compute_Tail_Area(&mut self) {
    //Computation of all intermediate k-constants
    let k_1_a:f64 = self.s_m*self.c_hr_a - self.x_ac_a;
    let k_1_b:f64 = self.x_ac_b -self.s_m*self.c_hr_a; 
    self.k_2_u = self.m_f*self.x_cg_f + self.m_tb*self.x_cg_tb + self.m_tm*self.x_cg_tm; 
    self.k_2_l = self.m_f + self.m_tb + self.m_tm; 
    let k_3_lc:f64 = k_1_a*self.k_2_l + self.k_2_u; 
    let k_3_ld:f64 = k_1_a + self.x_cg_wb; 
    let k_3_rc:f64 = k_1_b*self.k_2_l - self.k_2_u; 
    let k_3_rd:f64 = k_1_b - self.x_cg_wb; 
    let k_4_a:f64 = self.c_L_alpha_b*k_3_rd*self.c_bs; 
    let k_4_b:f64 = self.c_L_alpha_b*(k_3_rc+k_3_rd*self.c_bf)-self.A_w_a*self.c_L_alpha_a*k_3_ld*self.c_bs; 
    let k_4_c:f64 =  - self.A_w_a*self.c_L_alpha_a*(k_3_lc+k_3_ld*self.c_bf); 
    //Computation of the Tail Wing Area and return this value
    self.A_w_b = (-k_4_b + (k_4_b.powf(2.0)-4.0*k_4_a*k_4_c).powf(0.5))/(2.0*k_4_a);}

/*Computes the mass of wing B, implements linear approximation with area*/
pub fn Tail_Wing_Mass(&mut self){self.m_wb = self.c_bs*self.A_w_b+self.c_bf;}

/*Based on wing B's area and for level steady flight, compute Lift at wing B*/
pub fn Tail_Downforce(&mut self) {
    self.Tail_Wing_Mass();
    self.x_cg_o = (self.k_2_u+self.m_wb*self.x_cg_wb)/(self.k_2_l+self.m_wb);
    self.m_tot = self.m_f + self.m_wb + self.m_tb + self.m_tm;
    self.q = (self.m_tot*g)/(self.A_w_a*self.c_L_0_a + (self.A_w_a*self.c_L_0_a*(self.x_cg_o-self.x_ac_a) + self.A_w_a*self.c_hr_a*self.c_M_ac_a + self.A_w_b*self.c_hr_b*self.c_M_ac_b)/(self.x_ac_b-self.x_cg_o));
    self.trim_vel = q2vel(self.q);
    self.L_0_a = cL2L(self.c_L_0_a, self.q, self.A_w_a);
    self.L_0_b = self.m_tot*g - self.L_0_a;}
}
