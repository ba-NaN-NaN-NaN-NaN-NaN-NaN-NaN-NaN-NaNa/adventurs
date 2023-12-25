use rand::Rng;
use rand::rngs::ThreadRng;

#[allow(dead_code)]
pub struct Blizzard {
    hails_orig: Vec<Hail>,
}

const D24_EPSILON: f64 = 1.0e-15f64;

#[allow(dead_code)]
impl Blizzard {
    pub fn from_content(content: &str) -> Blizzard {
        let unformatted: Vec<&str> = content.trim().split("\n").collect();
        let mut tiles:Vec<Hail> = Vec::new();

        for line in unformatted {
            let trimmed = line.trim();
            if trimmed.len() == 0 {
                continue;
            }
            tiles.push(Hail::from_content(trimmed));
        }
        let toreturn = Blizzard { 
            hails_orig: tiles,
        };

        toreturn
    }

    pub fn scale_all_hails(&mut self, scale: f64) {
        for hail in self.hails_orig.iter_mut() {
            hail.scale(scale);           
        }
    }

    pub fn part1(&mut self, test_area_min:f64, test_area_max:f64 ) -> i64 {
        let hail_count = self.hails_orig.len();
        let mut toreturn = 0;
        for i in 0..hail_count {
            for j in i..hail_count {
                if i==j {continue;}

                let hail_a = &self.hails_orig[j];
                let hail_b = &self.hails_orig[i];

                let intersection:Option<(f64, f64)> = hail_b.intersect_point_p1(hail_a);

                if let Some((inter_x, inter_y)) = intersection {
                    if inter_x <= test_area_min { continue; }
                    if inter_y <= test_area_min { continue; }

                    if inter_x >= test_area_max { continue; }
                    if inter_y >= test_area_max { continue; }

                    let hail_a_moved_x = inter_x - hail_a.px;
                    let hail_b_moved_x = inter_x - hail_b.px;
                    if hail_a.dx * hail_a_moved_x < -D24_EPSILON {
                        // Movement direction signs differ, is in A:s past.
                        continue;
                    }

                    if hail_b.dx * hail_b_moved_x < -D24_EPSILON {
                        // Movement direction signs differ, is in B:s past.
                        continue;
                    }

                    toreturn += 1;
                }
            }
        }
        
        toreturn
        
    }

    #[allow(dead_code)]
    pub fn part2(&mut self, scale: f64) -> i64 {
        let mut rng = rand::thread_rng();
        let px_starting = 0.0;
        let py_starting = 0.0;
        let pz_starting = 0.0;
        let dx_starting = 0.0;
        let dy_starting = 0.0;
        let dz_starting = 0.0;

        let mut magnitude = 5.0;

        let mut stone = Hail {
            px: px_starting,
            py: py_starting,
            pz: pz_starting,
            dx: dx_starting,
            dy: dy_starting,
            dz: dz_starting,
            px_t1: 0.0,
            py_t1: 0.0,
            pz_t1: 0.0,
        };

        let mut err_to_beat= 0.0;
        if self.hails_orig.len() == 5 {
            stone = Hail {
                px: 24.1,
                py: 13.0,
                pz: 10.0,
                dx: -3.0,
                dy: 1.0,
                dz: 2.0,
                px_t1: 0.0,
                py_t1: 0.0,
                pz_t1: 0.0,
            };
        } else {

            // These values aquired by repeatedly running the test
            // then copy-pasting output back into the source code,
            // then running again, repeatedly with different error calculation and perturbation variants.

            err_to_beat = 12853733524.465914; 
            stone = Hail {
                   
               px: 349090036095520.0,
               py: 252501009482406.0,
               pz: 121385026849739.0,

               dx: -125.0,
               dy: 25.0,
               dz: 272.0,
               px_t1: 0.0,
               py_t1: 0.0,
               pz_t1: 0.0,
           };
           
           

           err_to_beat = 2323683778429.01; 
           err_to_beat = 13844691.819262667; 
           stone = Hail {
            px: 349084334634500.0,
            py: 252498326441926.0,
            pz: 121393830576313.97,
            dx: -125.00000000000003,
            dy: 25.000000000000004,
            dz: 272.00000000000006,
            px_t1: 349084334634375.0,
            py_t1: 252498326441951.0,
            pz_t1: 121393830576585.97,
        };
        
 stone = Hail {
    px: 349084334634500.0+54837.0,
    py: 252498326441926.0-62780237.0,
    pz: 121393830576313.97+592743.0,
    dx: -125.0000000000000,
    dy: 25.00000000000000,
    dz: 272.000000000000,
    px_t1: 349084334634375.0,
    py_t1: 252498326441951.0,
    pz_t1: 121393830576585.97,
};

err_to_beat = 16058112.560915146; 
err_to_beat = 9319.298082265928; 
err_to_beat = 3249.20383017533; 

err_to_beat = 299062237298400030000.0;
err_to_beat = 48846780912455540.0; 
// err_to_beat = 0.006000000728457705; 
err_to_beat = 0.05478734152684609; 
err_to_beat = 0.006002727897326151; 
err_to_beat = 0.006000024426798026; 
    err_to_beat = 0.00000003547763824462889; 
 stone = Hail {
    px: 349084334634500.06,
    py: 252498326441926.0,
    pz: 121393830576313.92,






    dx: -125.000,
    dy: 25.000,
    dz: 272.00,
    px_t1: 349084653436458.5,
    py_t1: 252498241203567.8,
    pz_t1: 121393130603447.63,
}


        }
        stone.fix_t1();
         
        
        // stone.scale(scale);    

        let mut challenger = stone.clone();
        challenger.px += 0.1;
        let mut stone_error = self.avg_err_squared(&stone, true);
        println!("Initial stone_error is {}", stone_error);

        let mut challenger_error = self.avg_err_squared(&challenger, false);
        println!("Initial challenger_error is {}", challenger_error);

        for _ in 0..2000000 {
            
            if challenger_error < stone_error {
                // println!("Improved error from {} to {}", stone_error, challenger_error);
                stone = challenger;
                stone_error = challenger_error;
            }
            
            challenger = stone.clone();
            // challenger.perturb(&mut rng, magnitude);
            challenger.perturb2(&mut rng, magnitude);

            challenger_error = self.avg_err_squared(&challenger, false);
        }

        println!("Ending stone error {} for stone {:#?};", stone_error, stone);
        println!("Ending challender error {} for stone {:#?};", challenger_error, challenger);


        let mut guesses: Vec<Hail> = Vec::new();
        let mut avg_guess: Hail = Hail::from_content(&"0 0 0 @ 0 0 0".to_string());

        for _ in 0..10 {
            let stone0: usize = rng.gen_range(0..self.hails_orig.len());
            let stone1: usize = rng.gen_range(0..self.hails_orig.len());
            if stone0 == stone1 {continue;}
            guesses.push(self.guess_stone_velocities(&stone, stone0, stone1));         
        }

        for guess in guesses.iter() {
            avg_guess.px += guess.px;
            avg_guess.py += guess.py;
            avg_guess.pz += guess.pz;
            avg_guess.dx += guess.dx;
            avg_guess.dy += guess.dy;
            avg_guess.dz += guess.dz;
        }

        let guess_count: i64 = guesses.len().try_into().unwrap();
        let guess_count_f: f64 = guess_count as f64;

        avg_guess.px /= guess_count_f;
        avg_guess.py /= guess_count_f;
        avg_guess.pz /= guess_count_f;
        avg_guess.dx /= guess_count_f;
        avg_guess.dy /= guess_count_f;
        avg_guess.dz /= guess_count_f;
        avg_guess.px_t1 = avg_guess.px + avg_guess.dx;
        avg_guess.py_t1 = avg_guess.py + avg_guess.dy;
        avg_guess.pz_t1 = avg_guess.pz + avg_guess.dz;

        let error_for_average = self.avg_err_squared(&avg_guess, false);

        if error_for_average < err_to_beat {
            println!("Average guess with '{}' sum error. \n err_to_beat = {}; \n stone = {:#?}", error_for_average, error_for_average, avg_guess);
        }   
        
       
        // self.sum_of_errsquares(&stone, true);
        -1
    }


    pub fn guess_stone_velocities(&self, stone: &Hail, hail0_nr: usize, hail1_nr: usize) -> Hail {
        
        let hail0 = self.hails_orig[hail0_nr].clone();
        let hail1 = self.hails_orig[hail1_nr].clone();

        let collision_0 = stone.closest_distance(&hail0);
        let c0_dist = collision_0.0;
        let c0_stone_t = collision_0.1;
        let c0_hail_t = collision_0.2;
        let c0_stone_pos = stone.point_at_t(c0_stone_t);
        let c0_hail_pos = hail0.point_at_t(c0_hail_t);

        let collision_1 = stone.closest_distance(&hail1);
        let c1_dist = collision_1.0;
        let c1_stone_t = collision_1.1;
        let c1_hail_t = collision_1.2;
        let c1_stone_pos = stone.point_at_t(c1_stone_t);
        let c1_hail_pos = hail1.point_at_t(c1_hail_t);

        println!("c0_dist = {}, c0_stone_t = {}, c0_hail_t = {}, \n   c0_stone_pos = {:?}, \n   c0_hail_pos = {:?}", c0_dist , c0_stone_t , c0_hail_t , c0_stone_pos , c0_hail_pos);
        println!("c1_dist = {}, c1_stone_t = {}, c1_hail_t = {}, \n   c1_stone_pos = {:?}, \n   c1_hail_pos = {:?}", c1_dist , c1_stone_t , c1_hail_t , c1_stone_pos , c1_hail_pos);

        let stone_dt = c1_hail_t - c0_hail_t;
        let stone_dx_total = c1_hail_pos.0 - c0_hail_pos.0;
        let stone_dy_total = c1_hail_pos.1 - c0_hail_pos.1;
        let stone_dz_total = c1_hail_pos.2 - c0_hail_pos.2;

        let stone_dx_norm = stone_dx_total / stone_dt;
        let stone_dy_norm = stone_dy_total / stone_dt;
        let stone_dz_norm = stone_dz_total / stone_dt;

        println!("Deltas are {} {} {}", stone_dx_norm, stone_dy_norm, stone_dz_norm);

        // old_loc = new_loc - speed * time
        let stone_px_orig = c1_hail_pos.0 - c1_stone_t * stone_dx_norm;
        let stone_py_orig = c1_hail_pos.1 - c1_stone_t * stone_dy_norm;
        let stone_pz_orig = c1_hail_pos.2 - c1_stone_t * stone_dz_norm;

        let suggest: Hail = Hail {
            px: stone_px_orig,
            py: stone_py_orig,
            pz: stone_pz_orig,
            dx: stone_dx_norm,
            dy: stone_dy_norm,
            dz: stone_dz_norm,
            px_t1:0.0f64,
            py_t1:0.0f64,
            pz_t1:0.0f64,
        };
        // println!("Suggest\n{:#?}", suggest);
        suggest

    }



    pub fn avg_err_squared(&self, stone: &Hail, print_coords: bool) -> f64 {
        let mut toreturn = 0.0;


        for hail in self.hails_orig[2..20].iter() {
            let (dist, t1,t2) = hail.closest_distance(stone);
            /*
            if print_coords {
                println!("Stone {:?} collides with hail {:?} at distance {} at t={:?}=({:?})", stone, hail, dist, t1, hail.point_at_t(t1));
            }
             */
            // let dt = 0.0f64;
            // let dt = (t2-t1).abs();

            // toreturn += dist * dist + dt * dt;
            let hail_pos_at_t = hail.point_at_t(t1);
            let stone_pos_at_t = stone.point_at_t(t1);
            let err = (stone_pos_at_t.0 - hail_pos_at_t.0).abs().powi(2) + 
                            (stone_pos_at_t.1 - hail_pos_at_t.1).abs().powi(2) + 
                            (stone_pos_at_t.2 - hail_pos_at_t.2).abs().powi(2);

                            /*
            if err.abs() < 1.2 {
                println!("Collision is {} -> {:?}. err={}, dist={}. Total error is {}", t1, hail_pos_at_t, err, dist, toreturn);
            } */
                            
            if print_coords {
                println!("Stone {:?} collides with \n hail {:?} with err-distance {}", stone, hail, err);
                println!("hail_pos_at_t({}) -> {:?}", t1, hail_pos_at_t);
                println!("stone_pos_at_t({}) -> {:?}", t2, stone_pos_at_t);
                // println!("dt is {}, stone arrives {} before hail.", dt, t2-t1);
            }

            // This error calculation evolved repeatedly to try and tweak convergence speed +
            // keep in reasonable range of f64.
            // toreturn += err.abs() * err.abs() / 100000000000000.0;
            // toreturn += err.powf(1.1)/100000000000.0;
            // toreturn += err.abs() * err.abs() / 10000000000000000.0 + 0.1;
            toreturn += err.abs() * err.abs() / 10000.0;

        }

        let num_hails:f64 = self.hails_orig.len() as f64;
        toreturn / num_hails
    }

}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Hail {
    px: f64,
    py: f64,
    pz: f64,

    dx: f64,
    dy: f64,
    dz: f64,

    px_t1: f64,
    py_t1: f64,
    pz_t1: f64,
}

#[allow(dead_code)]
impl Hail {
    pub fn from_content(content: &str) -> Hail {
        let to_frag = content.replace( " @ ", " ").replace(", ", " ").replace("  ", " ");
        if to_frag.contains("  ") {
            println!("too many spaces in line '{}', trimmed to '{}'", content, to_frag);
            panic!("fdjhfd")
        };
        let frags: Vec<&str> = to_frag.split(" ").collect();
        let px = frags[0].parse::<f64>().unwrap();
        let py = frags[1].parse::<f64>().unwrap();
        let pz = frags[2].parse::<f64>().unwrap();
        let mut dx = frags[3].parse::<f64>().unwrap();
        let mut dy = frags[4].parse::<f64>().unwrap();
        let mut dz = frags[5].parse::<f64>().unwrap();

        /*
        dx *= 100.0;
        dy *= 100.0;
        dz *= 100.0;
         */

        let px_t1 = px + dx;
        let py_t1 = py + dy;
        let pz_t1 = pz + dz;

        /*
        let px_t1 = px + dx * 100.0; // This 100.0 multiplication constant makes the answer change from 27324 to 27328 due to floating point errors???
        let py_t1 = py + dy * 100.0;
        let pz_t1 = pz + dz * 100.0;
        */

        Hail { px, py, pz, dx, dy, dz, px_t1, py_t1, pz_t1 }
    }

    pub fn is_parallell_p1(&self, other: &Hail) -> bool {
        let x1 = self.px;
        let y1 = self.py;

        let x2 = self.px_t1;
        let y2 = self.py_t1;

        let x3 = other.px;
        let y3 = other.py;

        let x4 = other.px_t1;
        let y4 = other.py_t1;
        let denom = (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4);
        
        if denom.abs() < D24_EPSILON {
            return true
        }
         
        false
    }

    pub fn scale(&mut self, scale: f64) {
        self.px *= scale;
        self.py *= scale;
        self.pz *= scale;

        self.dx *= scale;
        self.dy *= scale;
        self.dz *= scale;
        self.fix_t1();

    }


    pub fn fix_t1(&mut self) {
        self.px_t1 = self.px + self.dx;
        self.py_t1 = self.py + self.dy;
        self.pz_t1 = self.pz + self.dz;

    }

    pub fn intersect_point_p1(&self, other: &Hail) -> Option<(f64, f64)> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line

        // L1 being defined by two distinct points (x1, y1) and (x2, y2)
        // L2 being defined by two distinct points (x3, y3) and (x4, y4).

        // The intersection P of line L1 and L2 can be defined using determinants. 
        //
        // Px = (x1y2 - y1x2)(x3 - x4) - (x1-x2)(x3y4 - y3x4)
        //         --------------------------------------------------------
        //         (x1-x2)(y3-y4) - (y1-y2)(x3-x4)
        //
        // Py = (x1y2 - y1x2)(y3-y4) - (y1-y2)(x3y4 - y3x4)
        //             -----------------------------------------------------
        //         (x1-x2)(y3-y4) - (y1-y2)(x3-x4)
        //
        // When the two lines are parallel or coincident, the denominator is zero. 

        // self.

        let denom = (self.px-self.px_t1)*(other.py-other.py_t1) - (self.py-self.py_t1)*(other.px-other.px_t1);

        
        if -D24_EPSILON < denom && denom < D24_EPSILON {
            println!("Failing intersect since denom is {}", denom);
            return None
        }
        
        let p_x = (self.px*self.py_t1 - self.py*self.px_t1)*(other.px-other.px_t1) - (self.px-self.px_t1)*(other.px*other.py_t1 - other.py*other.px_t1);
        let p_y = (self.px*self.py_t1 - self.py*self.px_t1)*(other.py-other.py_t1) - (self.py-self.py_t1)*(other.px*other.py_t1 - other.py*other.px_t1);

        Some((p_x / denom, p_y / denom))
    }

    pub fn intersect_point_p1b(&self, other: &Hail) -> Option<(f64, f64)> {
        // L1 being defined by two distinct points (x1, y1) and (x2, y2)
        // L2 being defined by two distinct points (x3, y3) and (x4, y4).

        let x1 = self.px;
        let y1 = self.py;

        let x2 = self.px_t1;
        let y2 = self.py_t1;

        let x3 = other.px;
        let y3 = other.py;

        let x4 = other.px_t1;
        let y4 = other.py_t1;



        // The intersection P of line L1 and L2 can be defined using determinants. 
        //
        // Px = (x1y2 - y1x2)(x3 - x4) - (x1-x2)(x3y4 - y3x4)
        //         --------------------------------------------------------
        //         (x1-x2)(y3-y4) - (y1-y2)(x3-x4)
        //
        // Py = (x1y2 - y1x2)(y3-y4) - (y1-y2)(x3y4 - y3x4)
        //             -----------------------------------------------------
        //         (x1-x2)(y3-y4) - (y1-y2)(x3-x4)
        let denom = (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4);
        

        /*
        if -D24_EPSILON < denom && denom < D24_EPSILON {
            println!("Failing intersect since denom is {}", denom);
            return None
        }
         */

        let p_x = (x1*y2 - y1*x2)*(x3 - x4) - (x1-x2)*(x3*y4 - y3*x4);
        let p_y = (x1*y2 - y1*x2)*(y3-y4) - (y1-y2)*(x3*y4 - y3*x4);

        Some((p_x / denom, p_y / denom))


    }

    pub fn perturb2(&mut self, rng: &mut ThreadRng, magnitude: f64) {
        // Manually adjust magnitude here from 500.0 down to 1.0 as we converge to the last digits in px,py,pz.
        self.px = rng.gen_range(self.px-1.0..self.px+1.0);
        self.py = rng.gen_range(self.py-1.0..self.py+1.0);
        self.pz = rng.gen_range(self.pz-1.0..self.pz+1.0);
        self.fix_t1()
    }


    pub fn perturb(&mut self, rng: &mut ThreadRng, magnitude: f64) {

        let do_time_or_pos: f64 = rng.gen();
        if do_time_or_pos > 0.5  {
            let dt_to_apply:f64 = rng.gen_range(-magnitude/2.0..magnitude/2.0);
            self.px += dt_to_apply * self.dx;
            self.py += dt_to_apply * self.dy;
            self.pz += dt_to_apply * self.dz;
        } else {
            self.px = rng.gen_range(self.px - magnitude/2.0..self.px + magnitude/2.0);
            self.py = rng.gen_range(self.py - magnitude/2.0..self.py + magnitude/2.0);
            self.pz = rng.gen_range(self.pz - magnitude/2.0..self.pz + magnitude/2.0);
            /*
            // Velocity converged somewhat quickly, recognized by velocities orbiting the same even number after repeated runs.
            // Uncomment this + manually enter ".0" on all velocity values to make positioning converge *much* quicker.
            self.dx = rng.gen_range(self.dx - magnitude/20.0..self.dx + magnitude/20.0);
            self.dy = rng.gen_range(self.dy - magnitude/20.0..self.dy + magnitude/20.0);
            self.dz = rng.gen_range(self.dz - magnitude/20.0..self.dz + magnitude/20.0);
             */
        }

        self.px_t1 = self.px + self.dx;
        self.py_t1 = self.py + self.dy;
        self.pz_t1 = self.pz + self.dz;
    }


    pub fn closest_distance(&self, other: &Hail) -> (f64, f64, f64) {
        // https://math.stackexchange.com/questions/2213165/find-shortest-distance-between-lines-in-3d
        let n = cross_product((self.dx, self.dy, self.dz), (other.dx, other.dy, other.dz));
        let n_norm_basis = n.0*n.0 + n.1*n.1 + n.2*n.2;
        // println!("Cross is {:?} with norm basis {}", n, n_norm_basis);


        // d = |n . (r1-r2)|
        //     -------------
        //        ||n||

        // ||n|| = sqrt(n . n)

        let dr = (other.px-self.px, other.py-self.py, other.pz-self.pz);
        // let n_dot_dr = (n.0*dr.0+n.1*dr.1+n.2*dr.2).abs();
        let n_dot_dr = dot_product(n, dr);
        // println!("for dr {:?}, n_dot_dr is {}", dr, n_dot_dr);



        let n_cube_abs = n_norm_basis.powf(1.0/2.0);
        // println!("n_cube_abs {}", n_cube_abs);
        let closest_distance = n_dot_dr / n_cube_abs;

        // ------------------------
        // Time difference:

        // t1 = (e2 X n) . (dr)
        //       -------------
        //           n . n
            
        // t2 = (e1 X n) . (dr)
        //       -----------
        //           n . n

        let e1 = (self.dx, self.dy, self.dz);
        let e2 = (other.dx, other.dy, other.dz);
        let e1_cross_n = cross_product(e1,n);
        let e2_cross_n = cross_product(e2,n);

        let n_dot_n = dot_product(n, n);

        let t1 = dot_product(e2_cross_n, dr);
        let t2 = dot_product(e1_cross_n, dr);

        (closest_distance.abs(), t1/n_dot_n, t2/n_dot_n)
    }

    pub fn point_at_t(&self, t: f64) -> (f64, f64, f64) {
        (self.px + t * self.dx,
         self.py + t * self.dy,
         self.pz + t * self.dz)
    }
}


// pub fn cross_product(ax:f64,ay:f64, az:f64, bx:f64, by:f64, bz:f64) -> (f64, f64, f64) {
pub fn cross_product( left: (f64, f64, f64), right: (f64, f64, f64)) -> (f64, f64, f64) {    
    let ax = left.0;
    let ay = left.1;
    let az = left.2;

    let bx = right.0;
    let by = right.1;
    let bz = right.2;

    let a1 = ax;
    let a2 = ay;
    let a3 = az;

    let b1 = bx;
    let b2 = by;
    let b3 = bz;

    // https://en.wikipedia.org/wiki/Cross_product#Computing
    let s1 = a2*b3 - a3*b2;
    let s2 = a3*b1 - a1*b3;
    let s3 = a1*b2 - a2*b1;

    (s1, s2, s3)

}

pub fn dot_product(left:(f64,f64,f64), right:(f64,f64,f64)) -> f64 {
    // let n_dot_dr = (n.0*dr.0+n.1*dr.1+n.2*dr.2).abs();
    left.0 * right.0 + left.1 * right.1 + left.2 * right.2
}

#[allow(dead_code)]
pub fn part1(content: &str,  test_area_min:f64, test_area_max:f64 ) -> i64 {
    let mut blizzard = Blizzard::from_content(&content);

    let toreturn = blizzard.part1(test_area_min, test_area_max);
    println!("Part 1 returning {}", toreturn);
    toreturn
}

#[allow(dead_code)]
fn part2(content: &str, scale: f64) -> i64 {
    let mut blizzard = Blizzard::from_content(&content);
    blizzard.scale_all_hails(scale);
    let toreturn = blizzard.part2(scale);
    println!("Part 2 returning {}", toreturn);
    // println!("Part 2 got {} result.", toreturn);
    toreturn
}



#[cfg(test)]
mod tests {
    // Run these tests using:
    // $ cargo test --package adventurs --bin adventurs -- y2023::d24::tests --nocapture

    use super::*;
    use crate::input;

    #[test]
    fn test_distance() {
        // https://math.stackexchange.com/questions/2213165/find-shortest-distance-between-lines-in-3d
        // (2,6,−9) and r2=(−1,−2,3) and the (non unit) direction vectors e1=(3,4,−4) and e2=(2,−6,1)
        let line1 = Hail::from_content("2, 6, -9 @ 3, 4, -4");
        let line2 = Hail::from_content("-1, -2, 3 @ 2, -6, 1");
        let (dist1,t1,t2) = line1.closest_distance(&line2);
        let (dist2,_,_) = line2.closest_distance(&line1);
        println!("Distances are {} {} with times {} {}", dist1, dist2, t1, t2);
        assert!(4.74 < dist1 && dist1 < 4.741);
        assert!(4.74 < dist2 && dist2 < 4.741);



        let rock = Hail::from_content("24, 13, 10 @ -3, 1, 2");
        let hail = Hail::from_content("19, 13, 30 @ -2, 1, -2");
        let (dist1,t1,t2) = rock.closest_distance(&hail);
        let (dist2,_,_) = hail.closest_distance(&rock);
        println!("Distances are {} {} with times {} {}", dist1, dist2, t1, t2);

    }


    #[test]
    fn test_p1_algebra() {
        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 18, 19, 22 @ -1, -1, -2
        // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
        let hail_a = Hail::from_content(&"19, 13, 30 @ -2, 1, -2");
        let hail_b = Hail::from_content(&"18, 19, 22 @ -1, -1, -2");
        assert!(!hail_a.is_parallell_p1(&hail_b));
        let intersect = hail_a.intersect_point_p1(&hail_b);
        if let Some((x, y)) = intersect {
            // println!("Got intersect {:?}", intersect.unwrap());
            assert!(14.3 < x && x < 14.4);
            assert!(15.3 < y && y < 15.4);
        } else {
            assert_eq!("Expected x=14.333, y=15.33, got None", "");
        }
        
        let intersect = hail_a.intersect_point_p1b(&hail_b);
        if let Some((x, y)) = intersect {
            // println!("Got intersect from p1b {:?}", intersect.unwrap());
            assert!(14.3 < x && x < 14.4);
            assert!(15.3 < y && y < 15.4);
        } else {
            assert_eq!("Expected x=14.333, y=15.33, got None", "");
        }

        
        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 20, 25, 34 @ -2, -2, -4
        // Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).
        let hail_a = Hail::from_content(&"19, 13, 30 @ -2, 1, -2");
        let hail_b = Hail::from_content(&"20, 25, 34 @ -2, -2, -4");
        assert!(!hail_a.is_parallell_p1(&hail_b));
        let intersect = hail_a.intersect_point_p1b(&hail_b);
        if let Some((x, y)) = intersect {
            // println!("Got intersect from p1b {:?}", intersect.unwrap());
            assert!(11.5 < x && x < 11.7);
            assert!(16.5 < y && y < 16.7);
        } else {
            assert_eq!("Expected x=11.667, y=16.667, got None", "");
        }

        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 12, 31, 28 @ -1, -2, -1
        // Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).
        let hail_a = Hail::from_content(&"19, 13, 30 @ -2, 1, -2");
        let hail_b = Hail::from_content(&"12, 31, 28 @ -1, -2, -1");
        assert!(!hail_a.is_parallell_p1(&hail_b));

        // Hailstone A: 19, 13, 30 @ -2, 1, -2
        // Hailstone B: 20, 19, 15 @ 1, -5, -3
        // Hailstones' paths crossed in the past for hailstone A.
        let hail_a = Hail::from_content(&"19, 13, 30 @ -2, 1, -2");
        let hail_b = Hail::from_content(&"20, 19, 15 @ 1, -5, -3");
        assert!(!hail_a.is_parallell_p1(&hail_b));

        // Hailstone A: 18, 19, 22 @ -1, -1, -2
        // Hailstone B: 20, 25, 34 @ -2, -2, -4
        // Hailstones' paths are parallel; they never intersect.
        let hail_a = Hail::from_content(&"18, 19, 22 @ -1, -1, -2");
        let hail_b = Hail::from_content(&"20, 25, 34 @ -2, -2, -4");
        // println!("Troubleshooting {:?} {:?}", hail_a, hail_b);
        assert!(hail_a.is_parallell_p1(&hail_b));

        // Hailstone A: 18, 19, 22 @ -1, -1, -2
        // Hailstone B: 12, 31, 28 @ -1, -2, -1
        // Hailstones' paths will cross outside the test area (at x=-6, y=-5).
        let hail_a = Hail::from_content(&"18, 19, 22 @ -1, -1, -2");
        let hail_b = Hail::from_content(&"12, 31, 28 @ -1, -2, -1");
        assert!(!hail_a.is_parallell_p1(&hail_b));

        // Hailstone A: 18, 19, 22 @ -1, -1, -2
        // Hailstone B: 20, 19, 15 @ 1, -5, -3
        // Hailstones' paths crossed in the past for both hailstones.
        let hail_a = Hail::from_content(&"18, 19, 22 @ -1, -1, -2");
        let hail_b = Hail::from_content(&"20, 19, 15 @ 1, -5, -3");
        assert!(!hail_a.is_parallell_p1(&hail_b));

        // Hailstone A: 20, 25, 34 @ -2, -2, -4
        // Hailstone B: 12, 31, 28 @ -1, -2, -1
        // Hailstones' paths will cross outside the test area (at x=-2, y=3).
        let hail_a = Hail::from_content(&"20, 25, 34 @ -2, -2, -4");
        let hail_b = Hail::from_content(&"12, 31, 28 @ -1, -2, -1");
        assert!(!hail_a.is_parallell_p1(&hail_b));

        // Hailstone A: 20, 25, 34 @ -2, -2, -4
        // Hailstone B: 20, 19, 15 @ 1, -5, -3
        // Hailstones' paths crossed in the past for hailstone B.
        let hail_a = Hail::from_content(&"20, 25, 34 @ -2, -2, -4");
        let hail_b = Hail::from_content(&"20, 19, 15 @ 1, -5, -3");
        assert!(!hail_a.is_parallell_p1(&hail_b));

        // Hailstone A: 12, 31, 28 @ -1, -2, -1
        // Hailstone B: 20, 19, 15 @ 1, -5, -3
        // Hailstones' paths crossed in the past for both hailstones.
        let hail_a = Hail::from_content(&"12, 31, 28 @ -1, -2, -1");
        let hail_b = Hail::from_content(&"20, 19, 15 @ 1, -5, -3");
        assert!(!hail_a.is_parallell_p1(&hail_b));

    }



    #[test]
    fn test_intersect_p1_formula() {
        // L1 being defined by two distinct points (x1, y1) and (x2, y2)
        // Px = (x1y2 - y1x2)(x3 - x4) - (x1-x2)(x3y4 - y3x4)
        //         --------------------------------------------------------
        //         (x1-x2)(y3-y4) - (y1-y2)(x3-x4)
        //
        // Py = (x1y2 - y1x2)(y3-y4) - (y1-y2)(x3y4 - y3x4)
        //             -----------------------------------------------------
        //         (x1-x2)(y3-y4) - (y1-y2)(x3-x4)

        let mut s = "
Px = (x1*y2 - y1*x2)*(x3-x4) - (x1-x2)*(x3*y4 - y3*x4)
        --------------------------------------------------------
	   (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4)

Py = (x1*y2 - y1*x2)*(y3-y4) - (y1-y2)*(x3*y4 - y3*x4)
         -----------------------------------------------------
	 (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4)
".to_string();

        // x1 = px_t1 = px+dx -->   dx = px_t1 - px
        // x2 = px

        /*
        s = s.replace("x1", "self.px_t1");
        s = s.replace("x2", "self.px");
        s = s.replace("x3", "other.px_t1");
        s = s.replace("x4", "other.px");
            
        s = s.replace("y1", "self.py_t1");
        s = s.replace("y2", "self.py");
        s = s.replace("y3", "other.py_t1");
        s = s.replace("y4", "other.py");

        s = s.replace("self.px_t1-self.px", "self.dx");
        s = s.replace("self.py_t1-self.py", "self.dy");
        s = s.replace("other.px_t1-other.px", "other.dx");
        s = s.replace("other.py_t1-other.py", "other.dy");
         */

        s = s.replace("x1", "self.px");
        s = s.replace("x2", "self.px_t1");
        s = s.replace("x3", "other.px");
        s = s.replace("x4", "other.px_t1");

        s = s.replace("y1", "self.py");
        s = s.replace("y2", "self.py_t1");
        s = s.replace("y3", "other.py");
        s = s.replace("y4", "other.py_t1");

        s = s.replace("self.px_t1-self.px", "self.dx");
        s = s.replace("self.py_t1-self.py", "self.dy");
        s = s.replace("other.px_t1-other.px", "other.dx");
        s = s.replace("other.py_t1-other.py", "other.dy");
 
        println!("Intercept formula 2D: \n{}\n", s);

    }


    #[test]
    fn test_p1p2() {
        // Try non-seperate tests.
        // Run as: 
        // cargo test --package adventurs --bin adventurs -- y2023::d24::tests --nocapture
        
        {
            let pbuf = input::get_input("2023_d24_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part1(&content, 7.0, 27.0);
            assert_eq!(steps, 2); // p1 sample
        }
        if false {
            let pbuf = input::get_input("2023_d24.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part1(&content, 200000000000000.0, 400000000000000.0);
            assert!(steps > 27324);
            
            assert_eq!(steps, 27328); // p1 skarp
        }

        if false {
            let pbuf = input::get_input("2023_d24_sample.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let steps = part2(&content, 1.0);
            assert_eq!(steps, 47); // p2 sample
        }
        {
            let pbuf = input::get_input("2023_d24.txt").unwrap();
            let content = input::readstring(&pbuf).unwrap();
            let actual = part2(&content, 1.0);           
            // 722976072427665 is too low.
            
            assert_eq!(actual, 3030303030303); // p2 skarp
        }
    }
}
