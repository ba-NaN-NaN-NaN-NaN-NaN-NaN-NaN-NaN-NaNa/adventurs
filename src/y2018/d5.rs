
#[allow(dead_code)]
pub fn reducer(s : Vec<u8>) -> Vec<u8> {
    // Reduce first pair of 'xX' or 'Xx' where x can be any char, but x and X must be different cases.

    if s.len() < 1 {
        return s.to_vec()
    }   
    
    for n in  0..(s.len() -1) {
        // print!("fdh {}", n);
        if s[n] != s[n+1] {
            let c1 = char::from(s[n]);
            let c2 = char::from(s[n+1]);

            let mut c1l = char::from(s[n]);
            c1l.make_ascii_lowercase();
            let mut c2l = char::from(s[n+1]);
            c2l.make_ascii_lowercase();

            if c1 != c2 && c1l == c2l  {
                //return s[0..n] + s[n+2..]
                // print!("REDUCING of {} and {} / {} and {}\n", c1, c2, c1l, c2l);
                let mut v = Vec::new();
                v.extend(s[0..n].iter().cloned());
                v.extend(s[n+2..].iter().cloned());
                return v
            }
            // print!("No reduction of {} and {} / {} and {}\n", c1, c2, c1l, c2l);
        }
    }

    return s.to_vec()
}