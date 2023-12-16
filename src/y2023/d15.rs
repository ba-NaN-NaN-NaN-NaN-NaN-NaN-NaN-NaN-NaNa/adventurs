#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Box {
    lenses: Vec<Lens>,
}

#[allow(dead_code)]
impl Box {
    pub fn new() -> Box {
        Box { lenses: Vec::new() }
    }

    pub fn remove_by_label(&mut self, label: &String) {
        for n in 0..self.lenses.len() {
            if self.lenses.get(n).unwrap().label == *label {
                self.lenses.remove(n);
                return
            }
        }
    }

    pub fn upsert(&mut self, label: &String, flen: i64) {
        for lens in self.lenses.iter_mut() {
            if lens.label == *label {
                // Update
                lens.flen = flen;
                return
            }
        }

        // insert
        self.lenses.push(Lens { label: label.to_string(), flen: flen });
    }

    pub fn formatted(&self, box_nr: i64) -> String {
        if self.lenses.len() == 0 {
            return "".to_string();
        }
        let mut frags: Vec<String> = Vec::new();
        frags.push(format!("Box {}:", box_nr));

        for lens in self.lenses.iter() {
            frags.push(format!("[{} {}]", lens.label, lens.flen));
        }

        frags.join(" ").to_string()
    }


    pub fn fpower(&self, box_nr: i64) -> i64 {
        // Focusing power of a box:
        // sum of focusing power of lenses.
        let mut toreturn:i64 = 0;

        // for lens in self.lenses.iter() {
        for slot_nr in 0..self.lenses.len() {
            // toreturn += lens.fpower(box_nr, slot_nr);
            let lens = self.lenses.get(slot_nr).unwrap();
            let slot_nr_i64: i64 = slot_nr.try_into().unwrap();

            
            // frags.push(format!("[{} {}]", lens.label, lens.flen));

            
            let x = box_nr + 1;
            let y = slot_nr_i64 + 1;
            let z = lens.flen;
            let fpower = x * y * z;
            
            if false {
                println!("fpower for lens {}: {}  * {} * {} (focal length) = {})",
                    lens.label,
                    x,y,z,
                    fpower,
                );
            }

            toreturn += fpower;

        }

        
        toreturn
    }

}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Facility {
    boxes: Vec<Box>,
}

impl Facility {
    pub fn new() -> Facility {
        let mut boxes: Vec<Box> = Vec::new();
        for _ in 0..256 {
            boxes.push(Box::new())
        }

        Facility {
            boxes:boxes,
        }
    }

    #[allow(dead_code)]
    pub fn do_operation(&mut self, op: &Operation, print_progress: bool) {
        if print_progress {
            println!("Factory: after '{}'", op.unparsed);
        }
        match op.opchar {
            OpChar::Dash => {
                self.remove_lens(&op.label);
            },
            OpChar::Equals => {
                self.upsert_lens(&op.label, op.flen);
            },
        }

        if print_progress {
            let formatted = self.formatted();
            println!("{}", formatted);
        }
    }

    #[allow(dead_code)]
    pub fn remove_lens(&mut self, label: &String) {
        let box_nr: usize = simple_hash(label).try_into().unwrap();
        self.boxes.get_mut(box_nr).unwrap().remove_by_label(label);

    }

    #[allow(dead_code)]
    pub fn upsert_lens(&mut self, label: &String, flen: i64) {
        let box_nr: usize = simple_hash(label).try_into().unwrap();
        self.boxes.get_mut(box_nr).unwrap().upsert(label, flen);
    }

    #[allow(dead_code)]
    pub fn formatted(&self) -> String {
        let mut frags: Vec<String> = Vec::new();

        if self.boxes.len() == 0 {
            return "".to_string()
        }

        
        for box_nr in 0..self.boxes.len() {
            let box_obj = self.boxes.get(box_nr).unwrap();
            let formatted = box_obj.formatted(box_nr.try_into().unwrap());
            let trimmed = formatted.trim().to_string();
            if trimmed.len() > 0 {
               frags.push(trimmed);
            }
        }

        frags.join("\n").to_string()
        
    }

    #[allow(dead_code)]
    pub fn fpower(&self) -> i64 {
        let mut toreturn = 0;
        for box_nr in 0..self.boxes.len() {
            let box_obj = self.boxes.get(box_nr).unwrap();
            toreturn += box_obj.fpower(box_nr.try_into().unwrap());
        }
        toreturn
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum OpChar {
    Equals,
    Dash,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Operation {
    unparsed: String,
    label_hash: i64, // OBS! Only of the label.
    label: String, // rn, qp, cm etc.
    opchar: OpChar, // - or = 
    flen: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Lens {
    label: String, // rn, qp, cm etc.
    flen: i64,
}

impl Operation {
    pub fn from(content: &str) -> Operation {
        let unparsed = content.trim().to_string();
        
        if content.ends_with("-") {
            let label = content[0..content.len()-1].to_string();
            Operation {
                unparsed: unparsed,
                label_hash: simple_hash(&label),
                label: label,
                opchar: OpChar::Dash, // - or = 
                flen: 0,     
            }
        } else {
            let last_char = content[content.len()-1..].to_string();
            let opchar_str = content[content.len()-2..content.len()-1].to_string();
            let label = content[0..content.len()-2].to_string();

            if opchar_str != "=" {
                println!("Can not parse operation based on {}", content);
            }
            Operation {
                unparsed: unparsed,
                label_hash: simple_hash(&label),
                label: label, // rn, qp, cm etc.
                opchar: OpChar::Equals, // - or = 
                flen: last_char.parse().unwrap(),     
            }
        }
        
        // The result of running the HASH algorithm on the label indicates the correct box for that step.

    }
}


#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let frags: Vec<&str> = input.trim().split(",").collect();
    let mut toreturn = 0;
    for frag in frags {
        toreturn += simple_hash(frag)
    }
    toreturn
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let frags: Vec<&str> = input.trim().split(",").collect();
    
    let print_progress = input.len() < 450;
    let mut f = Facility::new();
    for frag in frags {
        
        let op = Operation::from(frag);
        // println!("Got operation {:?}", op);
        f.do_operation(&op, print_progress);
    }
    f.fpower()
}

#[allow(dead_code)]
pub fn simple_hash(content: &str) -> i64 {
    // 

    /*
    Determine the ASCII code for the current character of the string.
    Increase the current value by the ASCII code you just determined.
    Set the current value to itself multiplied by 17.
    Set the current value to the remainder of dividing itself by 256.
    */
    let bytes = content.trim().as_bytes();
    let codes: Vec<i64> = bytes.iter().map( |b| *b as i64).collect();
    // println!("Converted str {} to bytevec {:?}", content, codes);

    let mut toreturn:i64 = 0;
    for code in codes {
        toreturn += code;
        toreturn = toreturn*17;
        toreturn = toreturn % 256;
    }
    toreturn
}




#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d15::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_simple() {
        // assert_eq!(simple_hash(" actual, 202020202); // p1 skarp
        assert_eq!(simple_hash("rn=1"), 30);
        assert_eq!(simple_hash("cm-"), 253);
        assert_eq!(simple_hash("qp=3"), 97);
        assert_eq!(simple_hash("cm=2"), 47);
        assert_eq!(simple_hash("qp-"), 14);
        assert_eq!(simple_hash("pc=4"), 180);
        assert_eq!(simple_hash("ot=9"), 9);
        assert_eq!(simple_hash("ab=5"), 197);
        assert_eq!(simple_hash("pc-"), 48);
        assert_eq!(simple_hash("pc=6"), 214);
        assert_eq!(simple_hash("ot=7"), 231);



    }
    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- --nocapture y2023::d15::tests 

        {
            let pbuf = input::get_input("2023_d15_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 1320); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d15.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part1(&content);
            assert_eq!(actual, 511416); // p1 skarp
        }
        {
            let pbuf = input::get_input("2023_d15_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 145); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d15.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content);
            assert_eq!(actual, 290779); // p2 skarp
        }
         
    }
}
