use g;use Sizing_Tail_Area;
impl Sizing_Tail_Area {

//c_hr_b, gotta change it to link with aspect ratio!!!

/*Supplemental Equations, mass of tail boom*/
pub fn Tail_Boom_Mass(&mut self){self.m_tb = self.L_tb*self.m_tb_L +self.m_tb_f}

/*Supplemental Equations, Linking some geometric variables to each other*/
pub fn Tail_Optimization_Var_Setup(&mut self){
    //Approximate mass of tail boom
    self.Tail_Boom_Mass();
    //Set location of center of gravity of tail boom to half its total length
    self.x_cg_tb = 0.5*self.L_tb;
    //Set aerodynamic center of wing B to edge of tail boom
    self.x_ac_b = self.L_tb;
    //set the center of mass of wing B to edge of tail boom
    self.x_cg_wb = self.L_tb;
    //Set the miscellaneous mass at the end of the tail
    self.x_cg_tm = self.L_tb; 
}

/*Cost function for the tail, minimize this function*/
pub fn Tail_Optimization_Cost_Function(&mut self){
    self.cost_val = g*(self.m_tb + self.m_wb) - self.L_0_b;}

//Bisection solving to determine the length of the optimum tail
//fn Tail_Optimization_Run(&mut self){}
}
