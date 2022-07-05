pub fn run() {
   
   // Control Flow
   // In Rust, control flows  specifically require boolean types
   let number: u32 = 50;
   let tup: (&str, i32) = ("A String", 22);
   if number < 50{
       println!{"Number smaller than 100 000"};
   } else if number == 40 {
       println!{"Number equals 40"};
   } else {
       println!{"Number larger than 40"};
       }

   // Single line control flow
   let number:i32 = if tup.1 > 20 {21} else {19};

}