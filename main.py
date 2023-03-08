#!/usr/bin/python3

def test():
    ar = []
    tempar = []
    for i in range(143, -1, -1):
        tempar.append(i)
        if i % 12 == 0:
            tempar.reverse()
            # ar.extend(tempar)
            for j in tempar:
                print(j, end=" ")
            print("\n")
            tempar = []
test()
