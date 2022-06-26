pub fn run() {
   
   // Control Flow
   // In Rust, control flows  specifically require boolean types
   let number: u32 = 100_000;
   let tup: (&str, i32) = ("A String", 22);
   if number < 100_000{
       println!{"Number smaller than 100 000"};
   } else if number == 100_000 {
       println!{"Number equals 100 000"};
   } else {
       println!{"Number larger than 100 000"};
       }

   // Single line control flow
   let number:i32 = if tup.1 > 20 {21} else {19};

}