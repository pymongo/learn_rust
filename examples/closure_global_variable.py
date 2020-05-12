def outer():
    state = False

    def inner():
        nonlocal state
        state = not state
        print(f"state = {state}")

    return inner


func = outer()
func()
func()
func()
