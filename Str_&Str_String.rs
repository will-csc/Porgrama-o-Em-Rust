fn main(){
    let str_type = String::new(); // Type 'str' which is
                                 // the "String buffer"
                                 
    let stc_string: &str = "william";
                                // Type "&str" which references
                                // to an address in memory, where
                                // the string is located
                                
    let dyn_string: String = String::from("william");
                                // String type is a dynamic
                                // string, represented by a 
                                // vector
}