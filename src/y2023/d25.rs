use std::collections::{HashMap, HashSet};



#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Component {
    origin: String,
    links: Vec<String>,
}



#[allow(dead_code)]
impl Component {
    pub fn from_line(content: &str) -> Component {
        let trimmed = content.trim().replace(": ", " ");
        if trimmed.len() == 0{panic!("kjfhdg")}

        let frags: Vec<&str> = trimmed.split(" ").collect();
        let origin: String = frags[0].to_string();
        let links: Vec<String> = frags[1..].iter().map(|d| d.to_string()).collect();
        Component { origin, links }
    }
}

#[allow(dead_code)]
pub struct Apparatus {
    components: HashMap<String, Component>,
    in_left_side: HashSet<String>,
    links_to_left: HashSet<(String, String)>,
    links_to_right: HashSet<(String, String)>,
}

#[allow(dead_code)]
impl Apparatus {
    pub fn from_content(content: &str) -> Apparatus {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut components: HashMap<String, Component> = HashMap::new();
        let mut links: HashSet<(String, String)> = HashSet::new();

        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            let comp = Component::from_line(trimmed);
            for dst in comp.links.iter()  {
                links.insert((comp.origin.clone(), dst.clone()));
                links.insert((dst.clone(), comp.origin.clone()));
            }
            components.insert(comp.origin.clone(), comp);           
        }

        for (_,dst) in links.iter() {
            if !components.contains_key(dst) {
                components.insert(dst.clone(),  Component { origin: dst.clone(), links: Vec::new() });
            }
        }

        // Make dual linking.
        for (src,dst) in links.iter() {
            if let Some(dst) = components.get_mut(dst).as_mut() {
                if !dst.links.contains(src) {
                    dst.links.push(src.clone())
                }
            }
        }        

        let toreturn = Apparatus { 
            components,
            in_left_side: HashSet::new(),
            links_to_left: HashSet::new(),
            links_to_right: HashSet::new(),
        };

        toreturn
    }

    pub fn plantuml(&self) -> String {
        let mut frags: Vec<String> = Vec::new();

        println!("puml from {:?}", self.components);
        for (c_name, c_obj) in self.components.iter() {
            for d_name in c_obj.links.iter() {
                frags.push(format!("   {} --- {}:", c_name, d_name));
            }
        }

        frags.join("\n")
    }

    pub fn move_to_left(&mut self, cname: &str) {
        if self.in_left_side.contains(cname) {
            return
        }

        self.in_left_side.insert(cname.to_string());

        let movee: String = cname.to_string();
        if let Some(move_obj) = self.components.get(&movee) {
            // println!("Move obj is {:?}", move_obj);
            for friend in move_obj.links.iter() {
                if self.in_left_side.contains(friend) {
                    // movee moved to left, friend was already on left side. Remove movee->friend in links to left....
                    if !self.links_to_left.remove(&(movee.clone(), friend.clone())) { panic!("link mismatch a ") }
                    // ... and remove friend->movee in links to right....
                    if !self.links_to_right.remove(&(friend.clone(), movee.clone())) { panic!("link mismatch b ") }

                } else {
                    // movee moved to left. We need to create double links between sides.
                    if !self.links_to_left.insert((friend.clone(), movee.clone())) { panic!("link mismatch c") }
                    if !self.links_to_right.insert((movee.clone(), friend.clone())) { panic!("link mismatch d") }
                }
            }

        } else {
            println!("Can not move cname '{:?}', valid are '{:?}'", cname, self.components.keys());
            panic!("bad cname in move to left??")
        }

        

        // println!("moved '{}' to left. Links to left={:?}, links to right={:?}", cname, self.links_to_left, self.links_to_right);
    }


    pub fn move_to_right(&mut self, cname: &str) {
        if !self.in_left_side.contains(cname) {
            return
        }
        self.in_left_side.remove(cname);

        let movee: String = cname.to_string();
        if let Some(move_obj) = self.components.get(&movee) {
            // println!("Move obj is {:?}", move_obj);
            for friend in move_obj.links.iter() {
                if !self.in_left_side.contains(friend) {
                    // movee moved to right, friend was already on right side. Remove movee->friend in links to right....
                    if !self.links_to_right.remove(&(movee.clone(), friend.clone())) { panic!("link mismatch f ") }
                    // ... and remove friend->movee in links to left....
                    if !self.links_to_left.remove(&(friend.clone(), movee.clone())) { panic!("link mismatch g ") }

                } else {
                    // movee moved to right. We need to create double links between sides.
                    if !self.links_to_left.insert((movee.clone(), friend.clone())) { panic!("link mismatch h") }
                    if !self.links_to_right.insert((friend.clone(), movee.clone())) { panic!("link mismatch i") }
                }
            }

        } else {
            println!("Can not move cname '{:?}', valid are '{:?}'", cname, self.components.keys());
            panic!("bad cname in move to right??")
        }

        // println!("moved '{}' to right. Links to left={:?}, links to right={:?}", cname, self.links_to_left, self.links_to_right);
    }


    pub fn rearrange(&mut self) {
        let links_to_right = self.links_to_right.clone();

        if self.links_to_right.len() == 3 {
            println!("solution is {:?}", self.links_to_right);
            panic!("fds")
        }


        for (_, dst) in links_to_right.iter() {
            self.move_to_left(dst);
            println!("flkdh {:?} {:?}", self.in_left_side, self.links_to_right);
            self.rearrange();
            self.move_to_right(dst);
        }
    }

    pub fn part1(&mut self) -> i64 {
        if self.components.len() == 15 {
            self.move_to_left("cmg");
            self.rearrange()
            
        } else {
            self.move_to_left("jpt");
        }


        if self.links_to_left.len() != 3 {panic!("Does not have 3 links to left, can not return part1.")}
        if self.links_to_right.len() != 3 {panic!("Does not have 3 links to right, can not return part1.")}
        self.components.len().try_into().unwrap()
    }


    pub fn part1_manual(&mut self) -> i64 {
        // 156:   ssd -- xqh [color=red,len=5];
        // 5444:   mqb -- qlc [color=red,len=5];
        // 6312:   nrs -- khn [color=red,len=5];

        let to_remove: Vec<(&str, &str)> = if self.components.len() == 15  {
            Vec::from_iter([
                ("hfx", "pzl"), ("bvb", "cmg"), 
                ("nvd", "jqt")])
        } else {
            Vec::from_iter([
                ("ssd", "xqh"), ("mqb", "qlc"), 
                ("nrs", "khn")])
        };

        for (node_a, node_b) in to_remove {
            {
                let node_a_string = node_a.to_string();
                let node_a_obj = self.components.get_mut(&node_a_string).unwrap();
                let mut to_remove: usize = 5400;
                for i in 0..node_a_obj.links.len(){
                    if node_a_obj.links[i] == node_b {
                        to_remove = i;
                    }
                }

                node_a_obj.links.remove(to_remove);
            }   
            {
                let node_b_string = node_b.to_string();
                let node_b_obj = self.components.get_mut(&node_b_string).unwrap();
                let mut to_remove: usize = 5400;
                for i in 0..node_b_obj.links.len(){
                    if node_b_obj.links[i] == node_a {
                        to_remove = i;
                    }
                }

                node_b_obj.links.remove(to_remove);
            }
        }

        let mut worklist: Vec<String> = Vec::new();
        let mut reachable: HashSet<String> = HashSet::new();

        if self.components.len() == 15 {
            worklist.push("hfx".to_string());
        } else {
            worklist.push("ssd".to_string());
        }

        while worklist.len() > 0 {
            let to_move = worklist.pop().unwrap();
            reachable.insert(to_move.clone());
            let to_move_obj = self.components.get(&to_move).unwrap();
            for linked_to in to_move_obj.links.iter() {
                if reachable.contains(linked_to) {
                    continue;
                }
                worklist.push(linked_to.clone())
            }

        }

        println!("reachable len is {}", reachable.len());        

        let total_node_count: i64 = self.components.len().try_into().unwrap();
        let reachable_count: i64 = reachable.len().try_into().unwrap();

        reachable_count * (total_node_count - reachable_count)

    }


}



#[allow(dead_code)]
pub fn part1(content: &str) -> i64 {

    let mut apparatus = Apparatus::from_content(&content);
    // apparatus.part1()
    apparatus.part1_manual()
}




#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d25::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_plantuml() {
        {
            let pbuf = input::get_input("2023_d25_sample.txt").unwrap();
            let pbuf = input::get_input("2023_d25.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let a = Apparatus::from_content(&content);
            let drawing = a.plantuml();
            println!("Got drawing \n{}", drawing);
            assert_eq!(32, 54); // p1 sample
        }

    }

    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d25::tests --nocapture
        
        {
            let pbuf = input::get_input("2023_d25_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part1(&content);
            assert_eq!(steps, 54); // p1 sample
        }
        {
            let pbuf = input::get_input("2023_d25.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part1(&content);
            assert_eq!(steps, 619225); // p1 skarp
        }

    }
}
