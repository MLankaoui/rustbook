fn main() {
    //ownership rules
    /*
    each value in rust has an owner
    there can only be one owner at a time
    when the owner goes out of scope, the value will be dropped
     */
    {
        let s = "hello";// s is just valid in this scope, it remains valid untill it gors out of scope
    }
}
