#!/usr/bin/python3
import re
pattern = re.compile(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)") 
q1 = 0
q2 = 0
q3 = 0
q4 = 0
x2 = 50
y2 = 51
out = open("pyres.txt","w")
with open("input.txt") as f:
    i = 0
    for line in f.readlines():
        m = pattern.match(line)
        if m:
            px=int(m.group(1))
            py=int(m.group(2))
            vx=int(m.group(3))
            vy=int(m.group(4))
            print(px,py,vx,vy)
            x1 = (px + vx * 100) % 101
            y1 = (py + vy * 100) % 103
            out.write(f"{i},{x1},{y1}\n")
            if x1 < x2 and y1 < y2:
                q1 += 1
            if x1 > x2 and y1 < y2:
                q2 += 1
            if x1 > x2 and y1 > y2:
                q3 += 1
            if x1 < x2 and y1 > y2:
                q4 += 1
            i = i + 1
        else:
            print("ups")
print(q1,q2,q3,q4)
print(q1*q2*q3*q4)