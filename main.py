def RemoveFromList(thelist, val):
    return [value for value in thelist if value != val]

def GetDic():
    try:
        dicopen = open("DL.txt", "r")
        dicraw = dicopen.read()
        dicopen.close()
        diclist = dicraw.split("\n")
        diclist = RemoveFromList(diclist, '')
        return diclist
    except FileNotFoundError:
        print("No Dictionary!")
        return 
    
def Word2Vect(word):
    l = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']
    v = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    w = word.lower()
    wl = list(w)
    for i in range(0, len(wl)):
        if wl[i] in l:
            ind = l.index(wl[i])
            v[ind] += 1
    return v

def Vect2Int(vect):
    pv = 0
    f = 0
    for i in range(0, len(vect)):
        wip = (vect[i]*(2**pv))
        f += wip
        pv += 4
    return f
    
def Ints2Dic(dic):
    d = {}
    for i in range(0, len(dic)):
        v = Word2Vect(dic[i])
        Int = Vect2Int(v)
        if Int in d:
            tat = d.get(Int)
            tat.append(dic[i])
            d[Int] = tat
        elif Int not in d:
            d[Int] = [dic[i]]
    return d
        
d = GetDic()
ind = Ints2Dic(d)

while True:
    s = input("Enter Scrambbled Word Here: ")
    v = Vect2Int(Word2Vect(s))
    tp = ind.get(v, 'Word Not in Dictionary.')
    print(tp)
    if tp == 'Word Not in Dictionary.':
        if input('Would you like to add it? [y/n]') == 'y' or input('Would you like to add it? [y/n]') == 'Y':
            wta = str(input('What is the word you would like to add? '))
            dicopen = open("DL.txt", "a")
            dicopen.write('\n')
            dicopen.write(wta)
            dicopen.close()
            d = GetDic()
            ind = Ints2Dic(d)
            print('Dictionary Updated')
    if input('Restart? [y/n]') == 'y' or input('Restart? [y/n]') == 'Y':
        pass
    else:
        break

