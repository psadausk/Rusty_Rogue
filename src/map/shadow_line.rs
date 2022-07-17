pub struct ShadowLine {
    pub shadows: Vec<Shadow>,
}

impl ShadowLine {
    pub fn new() -> ShadowLine {
        ShadowLine { shadows: vec![] }
    }

    pub fn is_full_shadow(&self) -> bool {
        self.shadows.len() == 1 
        && self.shadows[0].start == 0.0
        && self.shadows[0].end == 1.0
    }

    pub fn is_in_shadow(&self, projection: &Shadow) -> bool {
        for shadow in &self.shadows {
            if shadow.contains(projection) {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, shadow: Shadow) {        
        let mut index = 0;
        for curShadow in &self.shadows {
            if curShadow.start >= shadow.start {
                break;
            }
            index = index + 1;
        }

        
        let overlappingPrev = {
            if index > 0 && self.shadows[index - 1].end > shadow.start {
                Some(self.shadows[index - 1])
            } else {
                None
            }
        };

        let overlappingNext = {
            if index < self.shadows.len() && self.shadows[index].start < shadow.end {
                Some(self.shadows[index])
            } else {
                None
            }
        };

         match overlappingNext {
             Some(next) => {
                  match overlappingPrev {
                     Some(prev) => {
                        //prev.end = next.end;
                        self.shadows[index - 1].end = next.end;
                        self.shadows.remove(index);
                     }, None => {
                        self.shadows[index].start = shadow.start;
                     }
                  }
             },
             None => {
                 match overlappingPrev {
                     Some(prev) => {
                        self.shadows[index - 1].end = shadow.end;
                     },
                     None => {
                         self.shadows.insert(index, shadow);
                     }
                 }
             }
        };
    }    
}

pub struct Shadow {
    pub start: f32,
    pub end: f32,
}

impl Clone for Shadow {
    fn clone(&self) -> Shadow {
        *self
    }
}

impl Copy for Shadow { }

impl Shadow {
    pub fn new(start: f32, end: f32) -> Shadow {
        Shadow {
            start: start,
            end: end,
        }
    }

    pub fn contains(self, other: &Shadow) -> bool {
        self.start <= other.start && self.end >= other.end
    }    
}
