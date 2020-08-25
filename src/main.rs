#![feature(test)]

extern crate test;

use test::Bencher;

#[derive(Debug)]
struct Cythan {
    cases: Vec<usize>
}

impl Cythan{
    fn new(cases: Vec<usize>) -> Cythan {
        Cythan{
            cases
        }
    }

    #[inline]
    fn next(&mut self) {
        unsafe {
            let index = {
                let mut index = self.get_mut_value(0);
                *index += 2;
                *index
            };

            let (c2,c1) = self.get_both_values(index-2);
    
            self.set_value(c1,self.get_value(c2));
        }
    }

    #[inline]
    fn get_both_values(&self, index: usize) -> (usize,usize) {
        let mut i = self.cases.iter().skip(index);
        (*i.next().unwrap_or(&0),*i.next().unwrap_or(&0))
    }

    #[inline]
    fn get_value(&self, index:usize) -> usize {
        *self.cases.get(index).unwrap_or(&0)
    }

    #[inline]
    unsafe fn get_mut_value(&mut self, index:usize) -> &mut usize {
        if self.cases.len() <= index {
            self.cases.extend((self.cases.len()..index+1).map(|x| 0)); 
        }
        self.cases.get_unchecked_mut(index)
    }

    #[inline]
    fn set_value(&mut self, index:usize, value:usize) {
        if self.cases.len() <= index {
            self.cases.extend((self.cases.len()..index+1).map(|x| 0)); 
        }
        self.cases[index] = value;
    }
}

fn main() {
    let mut cythan = Cythan::new( vec![1,9,5,10,1,0,0,11,0,1,20,21] );

    println!("Cythan start:{:?}",cythan);
    for a in 0..10 {
        cythan.next();
        println!("Cythan iteration {}:{:?}",a,cythan)
    }
}

#[bench]
fn bench(b: &mut Bencher) {
    let mut cythan = Cythan::new( vec![1,9,5,10,1,0,0,11,0,1,20,21] );
    b.iter(||{
        cythan.next()
    });
    println!("{:?}",cythan);
}