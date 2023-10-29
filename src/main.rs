mod  my_mod{
    pub fn my_fn(){
        println!("my_fn() called");
    }

    fn my_fn1(){
        println!("my_fn1() called");
    }
    pub mod my_mod2{
        pub fn my_fn2(){
            println!("my_fn2() called");
        }
    }
}
fn main() {
    my_mod::my_fn();
    //my_mod::my_fn1(); //error: function `my_fn1` is private
    my_mod::my_mod2::my_fn2();
}
