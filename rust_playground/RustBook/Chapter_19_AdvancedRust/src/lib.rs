// Declarative Macro (simplified version of vec!)
#[macro_export]
macro_rules! vec {
    // pattern to match on with a code block associated with that pattern (with one arm here)
    // macros can have multiple arms, where we match against actual code vs values in regular match expressions
    
    ( $($x: expr),* ) => {
    // capture any values $() capture any value inside the paranthesis for use in the placement code
    // the pattern $: expr captures any rust expression and places it intp x
    // for vec![1,2,3] the expression will match 3-times! and execute the respective code
    
        {
            let mut temp_vec = Vec::new();
            // here comes the match executable, i.e. it states for every match, replicate this block of code
            // and replace the corresponding values that were provided as expressions
            $(
                temp_vec.push($x);
            )*
            
            // after running the macro the code would look like this:
            /*
            let mut temp_vec = Vec::new();
            temp_vec.push(1);
            temp_vec.push(2);
            temp_vec.push(3);
            temp_vec
            */

            temp_vec
        } 
    };
}
/*
use proc_macro;

// Custom derived macro
#[some_attribute]
// name of function will be the name of the procedural macro
// Tokens are the smallest elements of a program
pub fn some_name(input: TokenStream) -> TokenStream {
    //...
}
*/