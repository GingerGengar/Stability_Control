//Library Import Definitions
mod Common; mod Tail_Sizing_Vars; mod Tail_Area;
mod IO; mod Specific_Tail_Optimization;
use Common::*; use Tail_Sizing_Vars::*;

/*Entry Point of the Program*/
fn main() {//Initialize based on default variables
let mut sim = Sizing_Tail_Area{..Sizing_Tail_Area::default()};
//Some Dynamic Linking going on
sim.Tail_Optimization_Var_Setup();
//Run Simulations
sim.Compute_Tail_Area();
sim.Tail_Downforce();
sim.Tail_Optimization_Cost_Function();
sim.print_essentials();}
