import time

def max_actividades(hrs:list[(int,int)]):   #usando FonraGoloso (FonraGreedy)
    hasta:int=0
    total:int=0
    for i in range(len(hrs)):   #recorro horarios x orden horario-fin
        if(hrs[i][0]>=hasta):
            hasta=hrs[i][1]
            total+=1
    print(total)

tests:int=int(input())  #cant de tests a ejecutar   
for t in range(tests):
    acts:int=int(input()) #cant de actividades que se tienen en cuenta
    horarios:list[(int,int)]=[0]*acts   #creo una lista con acts-posiciones inicializadas en 0 (no hago append x temas de complejidad)
    for a in range(acts):
        horarios[a]=(int(input()),int(input())) #pregunto al usuario desde que hora a q hora es una actividad y se lo agrego a las actividades
    time_start= time.perf_counter()
    horarios.sort(key=lambda x:x[1])    #ordeno la lista segun la hora-fin ([1]) usando lambdas (thx PLP)
    max_actividades(horarios)   #busco la cant max de actividades que puedo hacer 
    time_end=time.perf_counter()
    total_time=time_end - time_start
    print("el tiempo fue:",total_time)




