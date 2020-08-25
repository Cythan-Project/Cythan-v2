/*!
 * The Cythan machine emulator librairie.
 * 
 * The Cythan machine is a mathematical Turing Complete computer.
 * The machine is composed of one vector. Each value of the vector is a positive integer, "pointing" to another value.
 * 
 * ### For every iteration of the machine
 * 
 *  - The first case (the pointer), is incremented by 2.
 * 
 *  - The 2 cases pointed by the first case before the incrementation is "executed", which mean that the case pointed by the first executed case will be copied over the case pointed by the second one.
 * 
 * For exemple, `1,5,3,0,0,999` will copied the content of the 5th case (999) into the 3rd one. The result after one iteration will be `3,5,3,999,0,999`
*/





#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
extern crate test;

#[cfg(feature = "nightly")]
use test::Bencher;

/// The main structure of the Cythan machine
#[derive(Debug)]
struct Cythan {
    cases: Vec<usize>
}


impl Cythan{
    /// The constructor of the Cythan machine
    /// `cases` is a vector that represent the base code of the Cythan machine
    /// ### Example
    /// ```rust
    /// let mut cythan = Cythan::new( vec![1,9,5,10,1,0,0,11,0,1,20,21] );
    /// println!("Cythan start:{:?}",cythan);
    /// for a in 0..10 {
    ///    cythan.next();
    ///    println!("Cythan iteration {}:{:?}",a,cythan)
    /// }
    /// ```
    /// will make an if statement in Cythan, which will jump the pointer to either 20 or 21 accordingly if the 9th case contains an one or a zero.
    pub fn new(cases: Vec<usize>) -> Cythan {
        Cythan{
            cases
        }
    }

    /// Will execute one iteration of the Cythan machine.
    /// This method use a step of 2.
    #[inline]
    pub fn next(&mut self) {
        unsafe {
            let index = {
                let index = self.get_mut_value(0);
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

    /// Use this to get a value at an index in the Cythan machine.
    /// This will return 0 if the index doesn't exists.
    #[inline]
    pub fn get_value(&self, index:usize) -> usize {
        *self.cases.get(index).unwrap_or(&0)
    }

    #[inline]
    unsafe fn get_mut_value(&mut self, index:usize) -> &mut usize {
        if self.cases.len() <= index {
            self.cases.extend((self.cases.len()..index+1).map(|_| 0)); 
        }
        self.cases.get_unchecked_mut(index)
    }

    /// Use this to set a value at an index inside of th Cythan machine.
    /// This will fill the machine with 0 if the index doesn't exists.
    #[inline]
    pub fn set_value(&mut self, index:usize, value:usize) {
        if self.cases.len() <= index {
            self.cases.extend((self.cases.len()..index).map(|_| 0)); 
            self.cases.push(value);
        } else {
            unsafe {
                *self.cases.get_unchecked_mut(index) = value;
            }
        }
    }
}

#[cfg(feature = "nightly")]
#[bench]
fn bench(b: &mut Bencher) {
    let mut cythan = Cythan::new( vec![1,9,5,10,1,0,0,11,0,1,20,21] );
    b.iter(||{
        cythan.next()
    });
    println!("{:?}",cythan);
}