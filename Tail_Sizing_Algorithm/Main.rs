//List of Global Constants
const g:f64 = 9.81; //Gravitational constant (m/s^2)
const rho:f64 = 1.225; //Density of air (kg/m^3)

/*Given dynamic pressure, compute velocity*/
fn q2vel(q:f64)->f64 {((2.0*q)/rho).powf(0.5)}

/*Given Lift coeff and Dynamic pressure, and area, compute lift force*/
fn cL2L(c_L:f64, q:f64, A:f64)->f64 {q*A*c_L}

/*All the Variables Needed for Tail Sizing*/
struct Sizing_Tail_Area {
    /////////////////////////////////////////////////////////////////////
    // Start of Problem Specific Variables
    /////////////////////////////////////////////////////////////////////
        //////////////////////////////////////////////////////////////////
        //Needed to size the tail of the aircraft
        //////////////////////////////////////////////////////////////////
        s_m:f64, //Desired Static Margin of the aircraft (dimless)
        c_hr_a:f64, //Chord length of the main wing (m)
        x_ac_a:f64, //Aerodynamic Center of main wing (m)
        x_ac_b:f64, //Aerodynamic Center of wing B (tail wing) (m)
        m_f:f64, //The total mass of the front fuselage (kg)
        x_cg_f:f64, //Center of Gravity of front fuselage (m)
        m_tb:f64, //Mass of the Tail Boom (kg)
        x_cg_tb:f64, //Location of Center of Gravity of Tail Boom (m)
        m_tm:f64, //Mass of the other things on the tail (kg)
        x_cg_tm:f64, //Location of the center of gravity of the other things on tail (m)
        x_cg_wb:f64, //Center of Gravity of wing B (tail wing) (m)
        c_L_alpha_b:f64, //Derivative of Lift coefficient  for wing B (Tail Wing) (dimless)
        c_L_alpha_a:f64, //Derivative of Lift coefficient  for wing A (Main Wing) (dimless)
        c_bf:f64, //Fixed Constant for relating Area to Mass of wing B
        c_bs:f64, //Fixed Constant for relating Area to Mass of wing B
        A_w_a:f64, //Area of wing A (Main Wing) (m^2)
        //////////////////////////////////////////////////////////////////
        // Needed to Figure out lift force of tail
        //////////////////////////////////////////////////////////////////
        c_L_0_a:f64, //Lift coefficient at trim conditions for main wing
        c_M_ac_a:f64, //Moments about the aerodynamic center of main wing
        c_hr_b:f64, //chord length of tail wing
        c_M_ac_b:f64, //Moments about the aerodynamic center of tail wing
    /////////////////////////////////////////////////////////////////////
    // Start of Problem-Derived Variables
    /////////////////////////////////////////////////////////////////////
    //Communicable intermediate k-constants
    k_2_u:f64, k_2_l:f64, 
    m_wb:f64, //Weight of wing B (kg)
    A_w_b:f64, //Area of wing B (m^2)
    x_cg_o:f64, //Overall Center of Gravity Location (m)
    m_tot:f64, //Total Weight of the aircraft (kg)
    q:f64, //Dynamic Pressure of aircaft (Pa)
    trim_vel:f64, //Trim velocity of aircraft (m/s)
    L_0_a:f64, //Lift force on tail wing at trim flight
    L_0_b:f64, //Lift force on tail wing at trim flight
}


impl Sizing_Tail_Area {
fn default() -> Sizing_Tail_Area {
    Sizing_Tail_Area{//Default values for Aircraft Tail Sizing
    s_m: 0.2, //Desired Static Margin of the aircraft (dimless)
    c_hr_a: 0.4, //Chord length of the main wing (m)
    x_ac_a: 0.1, //Aerodynamic Center of main wing (m)
    x_ac_b: 1.9, //Aerodynamic Center of wing B (tail wing) (m)
    m_f: 1.2, //The total mass of the front fuselage (kg)
    x_cg_f: -0.012, //Center of Gravity of front fuselage (m)
    m_tb: 0.2, //Mass of the Tail Boom (kg)
    x_cg_tb: 0.3, //Location of Center of Gravity of Tail Boom (m)
    m_tm: 0.0, //Mass of the other things on the tail (kg)
    x_cg_tm: 0.0, //Location of the center of gravity of the other things on tail (m)
    x_cg_wb: 0.9, //Center of Gravity of wing B (tail wing) (m)
    c_L_alpha_b: 0.1, //Derivative of Lift coefficient  for wing B (Tail Wing) (dimless)
    c_L_alpha_a: 0.2, //Derivative of Lift coefficient  for wing A (Main Wing) (dimless)
    c_bf: 0.1, //Fixed Constant for relating Area to Mass of wing B
    c_bs: 0.2, //Fixed Constant for relating Area to Mass of wing B
    A_w_a: 2.0, //Area of wing A (Main Wing) (m^2)
    c_L_0_a: 1.0, //Lift coefficient at trim conditions for main wing
    c_M_ac_a: -0.2, //Moments about the aerodynamic center of main wing
    c_M_ac_b: 0.0, //Moments about the aerodynamic center of tail wing
    //Garbage valus for intermediate variables that need to communicate
    k_2_u: 1e-3, k_2_l: 1e-3, m_wb: 1e-3, A_w_b: 1e-3, x_cg_o: 1e-3, m_tot: 1e-3, q : 1e-3,
    c_hr_b: 0.2, trim_vel: 1e-3, L_0_a: 1e-3, L_0_b: 1e-3,
    }}

fn Compute_Tail_Area(&mut self) {
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

fn Tail_Downforce(&mut self) {
    self.m_wb = self.c_bs*self.A_w_b+self.c_bf;
    self.x_cg_o = (self.k_2_u+self.m_wb*self.x_cg_wb)/(self.k_2_l+self.m_wb);
    self.m_tot = self.m_f + self.m_wb + self.m_tb + self.m_tm;
    self.q = (self.m_tot*g)/(self.A_w_a*self.c_L_0_a + (self.A_w_a*self.c_L_0_a*(self.x_cg_o-self.x_ac_a) + self.A_w_a*self.c_hr_a*self.c_M_ac_a + self.A_w_b*self.c_hr_b*self.c_M_ac_b)/(self.x_ac_b-self.x_cg_o));
    self.trim_vel = q2vel(self.q);
    self.L_0_a = cL2L(self.c_L_0_a, self.q, self.A_w_a);
    self.L_0_b = self.m_tot*g - self.L_0_a;
}

}


/*Entry Point of the Program*/
fn main() {

//Use default variables for the rest
let mut sim = Sizing_Tail_Area{..Sizing_Tail_Area::default()};

//Run Simulations
sim.Compute_Tail_Area();
sim.Tail_Downforce();

//Give me the Answer!!!
println!("Wing B Area = {}", sim.A_w_b);
println!("Mass of wing B = {}", sim.m_wb);
println!("Overall COG = {}", sim.x_cg_o);
println!("Trim Velocity = {}", sim.trim_vel);
println!("Mass of aircraft = {}", sim.m_tot);
println!("Lift of main wing = {}", sim.L_0_a);
println!("Lift of aft wing = {}", sim.L_0_b);

}
