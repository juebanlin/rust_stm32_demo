use inline_python::python;

fn main() {
    let who = "world";
    let n = 5;
    python! {
        import platform,os
        print("system:",platform.system())
        print("python version:",platform.python_version())
        for i in range('n):
            print(i, "Hello", 'who)
        print("Goodbye")
    }
}