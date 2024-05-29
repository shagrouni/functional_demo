module main

printfn "Hello from F#"

let add x y =
   x + y 

let multiply x  y =
    x * y   

//1   
let a = add 1 1
printfn "%d" a 

//2
let add2 = (fun x y -> x + y)
let b = add2 2 2
printfn "%d" b 

//3
let c = (fun x y -> x + y) 3 3
printfn "%d" c 

//4
printfn "%d" ((fun a b -> a + b) 4 4) 

//5 send function
let ApplyFunc f x  y =
   f x y 

let d = ApplyFunc add 5 5
printfn "%d" d 

let e = ApplyFunc multiply  6 6
printfn "%d" e 

let f = ApplyFunc (fun x y -> x * y + 1)  7 7
printfn "%d" f 

//6 return function
let returnFunc x =
   if x + x >= 15 then
      fun x y -> x + y
   else
      fun x y -> x * y
    
let myFun = returnFunc 8
let g = myFun 8 8
printfn "%d" g 
