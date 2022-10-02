use Sizing_Tail_Area;

impl Sizing_Tail_Area {
/*Print all variables we are interested in*/
pub fn print_essentials(&mut self){
//println!("", );
    println!("####################################################################");
    println!("# Global Information about Aircraft:");
    println!("####################################################################");
    println!("Total Aircraft Mass = {} kg", self.m_tot);
    println!("Location of Overall Center of Gravity = {} m", self.x_cg_o);
    println!("Aircraft Trim Velocity = {} m/s", self.trim_vel);
    println!("####################################################################");
    println!("# Print Detail about Tail Boom");
    println!("####################################################################");
    println!("Tail Boom Length: {} m", self.L_tb);
    println!("Tail Boom Mass: {} kg", self.m_tb);
    println!("####################################################################");
    println!("# Print Details about wing B");
    println!("####################################################################");
    println!("Area of tail wing (wing B) = {} m^2", self.A_w_b);
    println!("Mass of tail wing (wing B) = {} kg", self.m_wb);
    println!("####################################################################");
    println!("# Lift Distribution");
    println!("####################################################################");
    println!("Lift produced at main wing (wing A) = {} N", self.L_0_a);
    println!("Lift produced at tail wing (wing B) = {} N", self.L_0_b);
    println!("Cost Function (Down Force of entire tail section) = {} N", self.cost_val);
    println!("####################################################################");
    println!("# End of Outputs");
    println!("####################################################################");}
}
