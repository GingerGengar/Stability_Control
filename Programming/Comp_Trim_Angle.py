#!/bin/python

#Library Import Statements
import numpy as np
import sys

#Name of Data File
Dat_File = str(sys.argv[1])

#Read in Data File
data = np.genfromtxt(Dat_File)

#Comutation of Lift to Drag Ratio
LDR = data[:,1]/data[:,2]

#Location of Maximum Lift to Drag Ratio
LDR_M_Loc = np.argmax(LDR)

#Maximum Lift to Drag Ratio
LDR_Max = LDR[LDR_M_Loc]

#Angle of Attack which Max L/D Occurs
alpha_LDR_Max = data[LDR_M_Loc, 0]

print("#Alpha L/D Max (deg) | L/D max Value")
print(alpha_LDR_Max, LDR_Max)
