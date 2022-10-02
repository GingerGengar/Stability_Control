#!/bin/python

#Library Import Statements
import numpy as np
import matplotlib.pyplot as plt
import sys

#Name of Data File
Dat_File = str(sys.argv[1])

#Read in Data File
data = np.genfromtxt(Dat_File)
A_r = 7.142857142857142
zero_loc = np.argmin(data[:,0]**2)
#This is the alpha values for the predicted line
alpha_pred = [data[zero_loc,0], data[-1,0]]
#This is the approximate derivative of the predicted lift curve
cl_grad_pred = 2*np.pi*(A_r/(A_r+2))
#This is predicted lift coeff at lower value of alpha, make it match
cl_lower = data[zero_loc, 1]
#This is higher predicted lift coeff at higher value of alpha
cl_upper = cl_lower + cl_grad_pred*np.deg2rad(alpha_pred[1]-alpha_pred[0])
#Forming the cl predicted array
cl_pred = [cl_lower, cl_upper]

#Plot Lift Coefficient Vs Alpha
figure_CL, axis_CL = plt.subplots()
axis_CL.plot(data[:,0], data[:,1], 'kx-',label='AVL Wing')
axis_CL.plot(alpha_pred, cl_pred, 'r--',label='Predicted')
axis_CL.set_xlabel('Angle of Atack')
axis_CL.set_ylabel('Lift Coefficient')
axis_CL.set_title('Lift Curve of a Wing')
axis_CL.legend()
axis_CL.grid()

#Plot Drag Curve
figure_CD, axis_CD = plt.subplots()
axis_CD.plot(data[:,0], data[:,2], 'rx-',label='Wing')
axis_CD.set_xlabel('Angle of Atack')
axis_CD.set_ylabel('Drag Coefficient')
axis_CD.set_title('Drag Curve of a Wing')
axis_CD.legend()
axis_CD.grid()

#Plot Drag Polar
figure_CDCL, axis_CDCL = plt.subplots()
axis_CDCL.plot(data[:,1], data[:,2], 'gx-',label='Wing')
axis_CDCL.set_xlabel('Lift Coefficient')
axis_CDCL.set_ylabel('Drag Coefficient')
axis_CDCL.set_title('Drag Polar of a Wing')
axis_CDCL.legend()
axis_CDCL.grid()

#Plot L/D Ratio
figure_LDR, axis_LDR = plt.subplots()
axis_LDR.plot(data[:,0], data[:,1]/data[:,2], 'cx-',label='Wing')
axis_LDR.set_xlabel('Angle of Attack (deg)')
axis_LDR.set_ylabel('L/D Ratio')
axis_LDR.set_title('L/D Ratio for a wing')
axis_LDR.legend()
axis_LDR.grid()

#Show Plots
plt.show()
