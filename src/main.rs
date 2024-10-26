use std::time::Instant;

fn main() {
    findevennumbers();
    findoddnumbers();
       findnegativeoddnumbers();
      
}


fn findevennumbers(){
  let start = Instant::now();
  println!("Even numbers from 1 to 10:");

  let mut d =1;
  let  c = 2;
  let mut b =0;
  while d <=10  {
    println!("{}", b);

      b = c * d;
     
      d+=1;
  } 
  

  let duration = start.elapsed();

  println!("Time elapsed: {:?}", duration); 

}

fn findoddnumbers() {

  let start = Instant::now();
  println!("Odd numbers from 1 to 10:");

  let mut d =1;
  let  c = 2;
  let mut b =0;
  while d <=10  {
    println!("{}", b);
      b = c * d + 1;
     

      d+=1;
  } 
  

  let duration = start.elapsed();

  println!("Time elapsed: {:?}", duration); 

    
}

fn findnegativeoddnumbers(){
  println!("Even numbers from -1 to 9:");
  let start = Instant::now();
  let mut c = -1;
  let mut odd = 0;
 
  while c >= -9 {
    
      println!("{}", odd);
       odd = 3 * c - 1 ; 
     c -= 1;
    
   }


  let duration = start.elapsed();

  
  println!("Time elapsed: {:?}", duration); 
}