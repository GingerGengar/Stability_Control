#!/bin/python

#Library import statements
import numpy as np
import sys

#This is the name of the specified input file
Input_File = str(sys.argv[1])
#This is what choice of airfoil to use
Airfoil_Selection = str(sys.argv[2])

#Reading in the data
data = np.genfromtxt(Input_File)

#Writing for incompressible FLows
print("Simple_Tapered_Wing")
print("#Mach\n", 0.0)
print("#IYsym   IZsym   Zsym\n", 0, 0, 0)
print("#Sref    Cref    Bref")

#WRiting Reference area, chord reference and span reference
print(data[0], end=" ") #Writing of Reference Area
print(data[2], end=" ") #Writing of Reference Chord
print(data[4]) #Writing of Reference Wing Span

#TODO:AUTOMATE THIS PART
#Writing Center of Mass (Unimportant)
print("#Xref    Yref    Zref\n", 0.232*data[2],0,0)

#TODO:AUTOMATE THIS PART
#Writing Drag obtained from XFoil
print("#CDp\n", 0.05597)

#Surface Header
print("#################################################################")
print("SURFACE")
print("WING")
print("#Nchordwise     Cspace  Nspanwise   Sspace")
print(30, 1.0, 30, 1.0)
print("YDUPLICATE\n", 0.0)
print("ANGLE\n", 0.0)
print("# x,y,z bias for whole surface")
print("TRANSLATE\n", 0.0, 0.0, 0.0)

#Definition of wing at the roots
print("#################################################################")
print("SECTION")
print("#   Xle     Yle     Zle     chord       angle       Nspan       Sspace")
print(0, 0, 0, data[2], 0, 30, 1.0)
print("AFIL\n", Airfoil_Selection)

#Definition of wing at the tips
print("#################################################################")
print("SECTION")
print("#   Xle     Yle     Zle     chord       angle       Nspan       Sspace")
print((data[2]-data[3])/2, data[4]/2, 0, data[3], 0, 30, 1.0)
print("AFIL\n", Airfoil_Selection)
