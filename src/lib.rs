/*!
 * The Cythan machine emulator librairy.
 *
 * The Cythan machine is a mathematical Turing Complete computer.
 * The machine is composed of one vector. Each value of the vector is a positive integer, "pointing" to another value.
 *
 * ### For every iteration of the machine
 *
 *  - The first case (the pointer), is incremented by 2.
 *
 *  - The 2 cases pointed by the first case before the incrementation is "executed". In a pair of executed cases, the case that as for index the second value is set to the value of the case that have as index the first value
 *
 * For instance, `1,5,3,0,0,999` will copy the content of the 5th case (999) into the 3rd one. The result after one iteration will be `3,5,3,999,0,999`
 *
 * ### Example
 *
 * ```rust
 * use cythan::Cythan;
 * let mut cythan = Cythan::new( vec![1,9,5,10,1,0,0,11,0,1,20,21] );
 * println!("Cythan start:{}",cythan);
 * for a in 0..10 {
 *    cythan.next();
 *    println!("Cythan iteration {}:{}",a,cythan)
 * }
 * ```
 *
*/

#![feature(test)]

extern crate test;

use test::Bencher;

use std::fmt;

/// The main structure of the Cythan machine
pub struct Cythan {
    cases: Vec<usize>,
    step: usize,
    generator: DefaultGenerator,
}

impl fmt::Display for Cythan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cythan(step: {}): {:?}", self.step, self.cases)
    }
}

impl Cythan {
    /// The constructor of the Cythan machine
    /// `cases` is a vector that represent the base code of the Cythan machine
    /// ### Example
    /// ```rust
    /// use cythan::Cythan;
    /// let mut cythan = Cythan::new( vec![1,9,5,10,1,0,0,11,0,1,20,21] );
    /// println!("Cythan start:{}",cythan);
    /// for a in 0..10 {
    ///    cythan.next();
    ///    println!("Cythan iteration {}:{}",a,cythan)
    /// }
    /// ```
    /// will make an if statement in Cythan, which will jump the pointer to either 20 or 21 accordingly if the 9th case contains an one or a zero.
    pub fn new(cases: Vec<usize>) -> Cythan {
        Cythan {
            cases,
            step: 2,
            generator:DefaultGenerator::FixedValue(0),
        }
    }

    pub fn new_config(
        cases: Vec<usize>,
        step: usize,
        generator: Box<dyn Fn(usize) -> usize>,
    ) -> Cythan {
        Cythan {
            cases,
            step,
            generator:DefaultGenerator::Function(generator),
        }
    }

    pub fn new_static_value(cases: Vec<usize>, step: usize, generator: usize) -> Cythan {
        Cythan {
            cases,
            step,
            generator:DefaultGenerator::FixedValue(generator),
        }
    }

    /// Will execute one iteration of the Cythan machine.
    /// This method use a step of 2.
    #[inline]
    pub fn next(&mut self) {
        unsafe {
            let step = self.step;
            let index = {
                let index = self.get_mut_value(0);
                *index += step;
                *index
            };

            let (c2, c1) = self.get_both_values(index - step);

            self.set_value(c1, self.get_value(c2));
        }
    }

    #[inline]
    fn get_both_values(&self, index: usize) -> (usize, usize) {
        let mut i = self.cases.iter().skip(index);
        (
            *i.next().unwrap_or(&(self.generator.generate(index))),
            *i.next().unwrap_or(&(self.generator.generate(index + 1))),
        )
    }

    /// Use this to get a value at an index in the Cythan machine.
    /// This will return 0 if the index doesn't exists.
    #[inline]
    pub fn get_value(&self, index: usize) -> usize {
        *self
            .cases
            .get(index)
            .unwrap_or(&(self.generator.generate(index)))
    }

    #[inline]
    unsafe fn get_mut_value(&mut self, index: usize) -> &mut usize {
        if self.cases.len() <= index {
            let iter = (self.cases.len()..index+1).map(|x| self.generator.generate(x)).collect::<Vec<usize>>();
            self.cases.extend(iter);
        }
        self.cases.get_unchecked_mut(index)
    }

    /// Use this to set a value at an index inside of th Cythan machine.
    /// This will fill the machine with 0 if the index doesn't exists.
    #[inline]
    pub fn set_value(&mut self, index: usize, value: usize) {
        if self.cases.len() <= index {
            let iter = (self.cases.len()..index).map(|x| self.generator.generate(x)).collect::<Vec<usize>>();
            self.cases.extend(iter);
            self.cases.push(value);
        } else {
            unsafe {
                *self.cases.get_unchecked_mut(index) = value;
            }
        }
    }
}

#[bench]
fn bench(b: &mut Bencher) {
    let mut cythan = Cythan::new(vec![1, 9, 5, 10, 1, 0, 0, 11, 0, 1, 20, 21]);
    b.iter(|| cythan.next());
    println!("{}", cythan);
}

enum DefaultGenerator {
    Function(Box<dyn Fn(usize) -> usize>),
    FixedValue(usize),
}

impl DefaultGenerator {
    #[inline]
    fn generate(&self,index:usize) -> usize {
        match self {
            DefaultGenerator::Function(fct) => (fct)(index),
            DefaultGenerator::FixedValue(f) => *f,
        }
    }
}

#[test]
fn test_if() {
    let mut cythan = Cythan::new(vec![1, 9, 5, 10, 1, 0, 0, 11, 0, 1, 20, 21]);
    for a in 0..10 {
        cythan.next();
    }
    assert_eq!(cythan.cases, vec![34, 20, 5, 10, 1, 1, 0, 11, 0, 1, 20, 21]);
}
#[test]
fn test_simple() {
    let mut cythan = Cythan::new(vec![1,5,3,0,0,999]);
    cythan.next();
    assert_eq!(cythan.cases, vec![3,5,3,999,0,999]);
}

#[test]
fn test_junk() {
    let mut cythan = Cythan::new_static_value(vec![1,0,10],2,3);
    cythan.next();
    assert_eq!(cythan.cases, vec![3, 0, 10, 3, 3, 3, 3, 3, 3, 3, 3]);
}

#[test]
fn test_double() {
    let mut cythan = Cythan::new_config(vec![1,],2,Box::new(|x| x*2));
    for a in 0..10 {
        cythan.next();
    }
    assert_eq!(cythan.cases, vec![21, 2, 4, 6, 12, 10, 12, 14, 16, 18, 20, 22, 20, 26, 28, 30, 28, 34, 36, 38, 44, 42, 44, 46, 48, 50, 52, 54, 60, 58, 60, 62, 64, 66, 68, 70, 68, 74, 76, 78, 80, 82, 84, 86, 76]);
}
