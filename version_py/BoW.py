a=[1,2,5,5] 
b=[5,5,5,5]

i:int=0
j:int=0
res:int=0 
while i<len(a) and j<len(a):
    n:int=a[i]-b[j]
    if (a[i]-b[j]) < -1:
        i+=1
    elif b[j] - a[i] <-1:
        j+=1
    else:
        i+=1
        j+=1
        res+=1
