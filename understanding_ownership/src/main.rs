fn main() {
    // Variable Scope
    {                      // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    
}
