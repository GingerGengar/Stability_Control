/*All the Variables Needed for Tail Sizing*/
pub struct Sizing_Tail_Area {
    /////////////////////////////////////////////////////////////////////
    // Start of Problem Specific Variables
    /////////////////////////////////////////////////////////////////////
        //////////////////////////////////////////////////////////////////
        // Needed for the tail boom of the aircraft
        //////////////////////////////////////////////////////////////////
        pub L_tb:f64, //Length of tail boom (m)
        pub m_tb_L:f64, //Mass per unit length of tail boom (kg/m)
        pub m_tb_f:f64, //Mass of tail boom that is fixed
        //////////////////////////////////////////////////////////////////
        // Needed to size the tail of the aircraft
        //////////////////////////////////////////////////////////////////
        pub s_m:f64, //Desired Static Margin of the aircraft (dimless)
        pub c_hr_a:f64, //Chord length of the main wing (m)
        pub x_ac_a:f64, //Aerodynamic Center of main wing (m)
        pub x_ac_b:f64, //Aerodynamic Center of wing B (tail wing) (m)
        pub m_f:f64, //The total mass of the front fuselage (kg)
        pub x_cg_f:f64, //Center of Gravity of front fuselage (m)
        pub m_tb:f64, //Mass of the Tail Boom (kg)
        pub x_cg_tb:f64, //Location of Center of Gravity of Tail Boom (m)
        pub m_tm:f64, //Mass of the other things on the tail (kg)
        pub x_cg_tm:f64, //Location of the center of gravity of the other things on tail (m)
        pub x_cg_wb:f64, //Center of Gravity of wing B (tail wing) (m)
        pub c_L_alpha_b:f64, //Derivative of Lift coefficient  for wing B (Tail Wing) (dimless)
        pub c_L_alpha_a:f64, //Derivative of Lift coefficient  for wing A (Main Wing) (dimless)
        pub c_bf:f64, //Fixed Constant for relating Area to Mass of wing B
        pub c_bs:f64, //Fixed Constant for relating Area to Mass of wing B
        pub A_w_a:f64, //Area of wing A (Main Wing) (m^2)
        //////////////////////////////////////////////////////////////////
        // Needed to Figure out lift force of tail
        //////////////////////////////////////////////////////////////////
        pub c_L_0_a:f64, //Lift coefficient at trim conditions for main wing
        pub c_M_ac_a:f64, //Moments about the aerodynamic center of main wing
        pub c_hr_b:f64, //chord length of tail wing
        pub c_M_ac_b:f64, //Moments about the aerodynamic center of tail wing
    /////////////////////////////////////////////////////////////////////
    // Start of Problem-Derived Variables
    /////////////////////////////////////////////////////////////////////
    //Communicable intermediate k-constants
    pub k_2_u:f64, pub k_2_l:f64, 
    pub m_wb:f64, //Weight of wing B (kg)
    pub A_w_b:f64, //Area of wing B (m^2)
    pub x_cg_o:f64, //Overall Center of Gravity Location (m)
    pub m_tot:f64, //Total Weight of the aircraft (kg)
    pub q:f64, //Dynamic Pressure of aircaft (Pa)
    pub trim_vel:f64, //Trim velocity of aircraft (m/s)
    pub L_0_a:f64, //Lift force on tail wing at trim flight
    pub L_0_b:f64, //Lift force on tail wing at trim flight
    pub cost_val:f64, //Value of the cost function that needs to be minimized
}

impl Sizing_Tail_Area {
/*Default values that are used for the variables*/
pub fn default() -> Sizing_Tail_Area {
    Sizing_Tail_Area{//Default values for Aircraft Tail Sizing
    L_tb: 1.6, //Length of tail boom (m)
    m_tb_L: 0.10, //Mass per unit length of tail boom (kg/m)
    m_tb_f: 0.01, //Mass of tail boom that is fixed
    s_m: 0.17, //Desired Static Margin of the aircraft (dimless)
    c_hr_a: 0.3556, //Chord length of the main wing (m)
    x_ac_a: 0.08249, //Aerodynamic Center of main wing (m)
    x_ac_b: 1.9, //Aerodynamic Center of wing B (tail wing) (m)                     [WARNING!!!]DL
    m_f: 2.8, //The total mass of the front fuselage (kg)
    x_cg_f: 0.1, //Center of Gravity of front fuselage (m)
    m_tb: 0.2, //Mass of the Tail Boom (kg)                                         [WARNING!!!]DL
    x_cg_tb: 0.3, //Location of Center of Gravity of Tail Boom (m)                  [WARNING!!!]DL
    m_tm: 0.0, //Mass of the other things on the tail (kg)
    x_cg_tm: 0.0, //Location of the center of gravity of other things on tail (m)   [WARNING!!!]DL
    x_cg_wb: 0.9, //Center of Gravity of wing B (tail wing) (m)                     [WARNING!!!]DL
    c_L_alpha_b: 0.050, //Derivative of Lift coefficient  for wing B (Tail Wing) (dimless)
    c_L_alpha_a: 0.07340, //Derivative of Lift coefficient  for wing A (Main Wing) (dimless)
    c_bf: 0.0, //Fixed Constant for relating Area to Mass of wing B
    c_bs: 1.12, //Fixed Constant for relating Area to Mass of wing B (kg/m^2)
    A_w_a: 0.903224, //Area of wing A (Main Wing) (m^2)
    c_L_0_a: 0.8, //Lift coefficient at trim conditions for main wing
    c_M_ac_a: -0.13048, //Moments about the aerodynamic center of main wing
    c_M_ac_b: 0.0, //Moments about the aerodynamic center of tail wing
    //Garbage valus for intermediate variables that need to communicate
    k_2_u: 1e-3, k_2_l: 1e-3, m_wb: 1e-3, A_w_b: 1e-3, x_cg_o: 1e-3, m_tot: 1e-3, q : 1e-3,
    c_hr_b: 0.2, trim_vel: 1e-3, L_0_a: 1e-3, L_0_b: 1e-3, cost_val: 1e-3, 
    }}
}
