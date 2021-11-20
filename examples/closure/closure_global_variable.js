function outer() {
    let state = false
    return function inner() {
        state = !state
        console.log(`state = ${state}`)
    }
}

let f = outer();
f() // true
f() // false
f() // true