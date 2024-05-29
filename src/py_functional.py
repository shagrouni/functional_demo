print("Hello from Python")

def add (x , y):
  return x + y

def multiply (x , y):
  return x * y

#1
a = add(1,1)
print(a)

#2
add2 = lambda x, y: x + y
b = add2(2, 2)
print (b)

#3
c = (lambda x, y: x + y)(3, 3)
print(c)

#4
print((lambda x, y: x + y)(4, 4))

#5 send function
def ApplyFunc (f, x , y):
  return f(x,y)

d = ApplyFunc(add, 5, 5)
print(d) 

e = ApplyFunc(multiply, 6, 6)
print(e) 

#6 return function
def returnFunc(x):
  if x + x >= 15:
    return lambda x, y: x + y
  else:
    return lambda x, y: x * y
    
myFun = returnFunc(7)
f = myFun(7, 7)
print (f)