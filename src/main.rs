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
    fn get_index(&self) -> usize {
        *self.cases.get(0).unwrap_or(&0)
    }
    fn next(&mut self) {
        self.set_value(0, self.get_index()+2); // ajoue de 2

        let c2 = self.get_value(self.get_index()-2);
        let c1 = self.get_value(self.get_index()-1);

        self.set_value(c1,self.get_value(c2)); // execution
    }

    fn get_value(&self, index:usize) -> usize {
        *self.cases.get(index).unwrap_or(&0)
    }

    fn set_value(&mut self, index:usize, value:usize) {
        if self.cases.len() <= index {
            for _ in self.cases.len()..index+1 {
                self.cases.push(0)
            }
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

#[bench] // use with cargo bench
    fn bench_add_two(b: &mut Bencher) {
        let mut cythan = Cythan::new( vec![1,9,5,10,1,0,0,11,0,1,20,21] );
        b.iter(||{cythan.next()}); // test fast cyth
    }