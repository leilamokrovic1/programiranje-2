use super::models::Sequence;
// Implementirajte artimetično zaporedje

pub struct Aritmetic {
    start : i64
    difference : i64
 }

impl Aritmetic {
    pub fn new(start: i64, difference: i64) -> Box<Aritmetic> {
        Box::new(Aritmetic{start, difference})
    }
}

impl Sequence<i64> for Aritmetic {
    fn name(&self) -> String {
        format! ("Aritmetic start={} difference={}", self.start, self. difference)
    }

    fn contains(&self, item: i64) -> bool {
        (item -self.start) % self.difference == 0
    }

    fn k_th(&self, k: i64) -> Option<i64> {
        Some(self.start + (k as i64)*self.difference)
    }
    
    fn start(&self) -> Option<i64> {
        Some(self.start)
    }

}
