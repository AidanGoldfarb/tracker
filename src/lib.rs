use std::collections::HashMap;

#[allow(dead_code)]
pub fn reuse_interval(trace: Vec<&str>) -> HashMap<usize, f32> {
    /*
        Init data structures
    */
    let mut im = HashMap::new(); // index map. k: trace element(address), v: last index element was read
    let mut rid = Vec::new(); //reuse interval distribution
    let mut rrid = Vec::new(); //real reuse interval distribution (rrd without inf.)

    let mut freq_map = HashMap::new(); //frequence map. k: reuse interval, v: frequence
    let mut dist_map = HashMap::new(); //distribution map. k: reuse interval, v: relative frequency
                                       /*
                                           End init data structures
                                       */

    /*
        (Real)Reuse interval distribution (rdd) calculation
    */
    for (i, val) in trace.iter().enumerate() {
        if !im.contains_key(val) {
            //first time seeing element, 0 instead of inf.
            im.insert(val, i);
            rid.push(0);
        } else {
            //revisiting element, need to update last index seen (equiv to caching element)
            let dist = i - im[val];
            rid.push(dist);
            rrid.push(dist);
            im.insert(val, i);
        }
    }
    /*
        End rdd calculation
    */

    let denom = rrid.len(); //denominator for relative frequency calculation. Number of non inf. reuse distances

    /*
        Begin distance frequency calc. May be able to be done in-place
    */
    for i in rrid {
        //Something like this may be better (as clippy suggests)
        // match freq_map.entry(&i){
        //     Some(v) => freq_map.insert(&i, freq_map[&i] + 1),
        //     None => freq_map.insert(&i, 1),
        // }

        if !freq_map.contains_key(&i) {
            freq_map.insert(i, 1);
        } else {
            freq_map.insert(i, freq_map[&i] + 1);
        }
    }
    /*
        End distance freq calc
    */

    /*
        Begin relative distribution frequency calculation
    */
    for (k, v) in freq_map.iter() {
        let freq: f32 = *v as f32 / denom as f32;
        dist_map.insert(*k, freq);
    }
    /*
        End relative distribution frequeny calculation
    */

    /*
        Return the relative distribution
    */
    dist_map
    /*
        End function
    */
}

#[allow(dead_code)]
fn calc_distance(window: Vec<&str>) -> usize {
    let mut res: usize = 1;
    let mut last = window.first().unwrap();
    for val in window.iter() {
        if val == last {
            continue;
        } else {
            res += 1;
            last = val;
        }
    }
    res
}
#[allow(dead_code)]
pub fn reuse_distance(trace: Vec<&str>) -> (HashMap<usize, usize>, HashMap<usize, f32>) {
    /*
        Init data structures
    */
    let mut im = HashMap::new(); // index map. k: trace element(address), v: last index element was read
    let mut rdd = Vec::new(); //reuse interval distribution
    let mut rrdd = Vec::new(); //real reuse interval distribution (rrd without inf.)

    let mut freq_map = HashMap::new(); //frequence map. k: reuse interval, v: frequence
    let mut dist_map = HashMap::new(); //distribution map. k: reuse interval, v: relative frequency

    // let mut freq_map = HashMap::new(); //frequence map. k: reuse interval, v: frequence
    // let mut dist_map = HashMap::new(); //distribution map. k: reuse interval, v: relative frequency
    /*
        End init data structures
    */

    /*
        (Real)Reuse interval distribution (rdd) calculation
    */
    for (i, val) in trace.iter().enumerate() {
        if !im.contains_key(val) {
            //first time seeing element, 0 instead of inf.
            im.insert(val, i);
            rdd.push(0);
        } else {
            let left = im[val];
            let dist = calc_distance(trace[left..i].to_vec());
            rdd.push(dist);
            rrdd.push(dist);
            im.insert(val, i);
        }
    }
    /*
        End rdd calculation
    */
    let denom = rrdd.len(); //denominator for relative frequency calculation. Number of non inf. reuse distances

    /*
        Begin distance frequency calc. May be able to be done in-place
    */
    for i in rrdd {
        //Something like this may be better (as clippy suggests)
        // match freq_map.entry(&i){
        //     Some(v) => freq_map.insert(&i, freq_map[&i] + 1),
        //     None => freq_map.insert(&i, 1),
        // }

        if !freq_map.contains_key(&i) {
            freq_map.insert(i, 1);
        } else {
            freq_map.insert(i, freq_map[&i] + 1);
        }
    }
    /*
        End distance freq calc
    */

    /*
        Begin relative distribution frequency calculation
    */
    for (k, v) in freq_map.iter() {
        let freq: f32 = *v as f32 / denom as f32;
        dist_map.insert(*k, freq);
    }
    /*
        End relative distribution frequeny calculation
    */

    /*
        Return the relative distribution
    */
    (freq_map, dist_map)
    /*
        End function
    */
}



// pub fn reuse_distance_eff(trace: Vec<&str>) -> HashMap<usize, usize> {//-> (HashMap<usize, usize>, HashMap<usize, f32>) {
//     let mut stack = Vec::new();
//     let mut freq_map: HashMap<usize,usize> = HashMap::new();

//     // a b c a b c a b a
//     for val in trace.iter(){
//         // println!("{:?}", stack);
//         if stack.contains(&val){ //resuse
//             let position = stack.iter().position(|&x| x == val).unwrap();  //get position in stack
//             if position == stack.len()-1{ //top of stack
//                 let freq = freq_map.entry(1).or_insert(0);
//                 *freq += 1;
//             }
//             else{
//                 let item = stack.remove(position);    //remove element and place at top
//                 stack.push(item);
//                 let temp_dist = stack.len()-position;
                
//                 let freq = freq_map.entry(temp_dist).or_insert(0);
//                 *freq += 1;
//             }
//         }
//         else{
//             stack.push(val);
//             // freq_map.insert(temp_dist, 0);
//         }
//     }
//     freq_map
// }

pub fn reuse_distance_eff(trace: Vec<String>, target: usize) -> HashMap<usize, usize> {//-> (HashMap<usize, usize>, HashMap<usize, f32>) {
    let mut stack = Vec::new();
    let mut freq_map: HashMap<usize,usize> = HashMap::new();

    // a b c a b c a b a
    for val in trace.iter(){
        // println!("{:?}", stack);
        if stack.contains(&val) && (**val).parse::<usize>().unwrap() == target{ //resuse
            //println!("val: {}", val);
            let position = stack.iter().position(|&x| x == val).unwrap();  //get position in stack
            if position == stack.len()-1{ //top of stack
                let freq = freq_map.entry(1).or_insert(0);
                *freq += 1;
            }
            else{
                let item = stack.remove(position);    //remove element and place at top
                stack.push(item);
                let temp_dist = stack.len()-position;
                //println!("DIST: {}\n", temp_dist);
                let freq = freq_map.entry(temp_dist).or_insert(0);
                *freq += 1;
            }
        }
        else if stack.contains(&val){
            let position = stack.iter().position(|&x| x == val).unwrap();
            let item = stack.remove(position);    //remove element and place at top
            stack.push(item);
        }
        else{
            stack.push(val);
            // freq_map.insert(temp_dist, 0);
        }
    }
    freq_map
}


#[allow(dead_code)]
pub fn dmd(dist: HashMap<usize, usize>) -> f32 {
    let mut res: f32 = 0.0;
    for (k, v) in dist.iter() {
        res += *v as f32 * (*k as f32).sqrt();
    }
    res
}

// pub fn dmd_from_trace(trace: Vec<&str>) -> f32 {
//     let freq_map = reuse_distance_eff(trace);
//     dmd(freq_map)
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn smoke() {
        assert!(1 == 1);
    }

    #[test]
    fn rd_cyclic() {
        let arr = vec!["a", "b", "c", "a", "b", "c"];
        let dist = reuse_distance_eff(arr);

        let mut map = HashMap::new();
        map.insert(3, 3);

        assert_eq!(dist, map);
    }

    // #[test]
    // fn ri_cyclic() {
    //     let arr = vec!["a", "b", "c", "a", "b", "c"];
    //     let dist = reuse_interval(arr);

    //     let mut map = HashMap::new();
    //     map.insert(3, 1.0);

    //     assert_eq!(dist, map);
    // }

    #[test]
    fn rd_sawtooth() {
        let arr = vec!["a", "b", "c", "c", "b", "a"];
        let dist = reuse_distance_eff(arr);

        let mut map = HashMap::new();
        map.insert(1 as usize, 1);
        map.insert(2 as usize, 1);
        map.insert(3 as usize, 1);

        assert_eq!(dist, map);
    }

    // #[test]
    // fn ri_sawtooth_distance_smoke() {
    //     let arr = vec!["a", "b", "c", "c", "b", "a"];
    //     let (_freq, _dist) = reuse_distance(arr);
    //     assert!(true);
    // }

    // #[test]
    // fn insert_sort() {
    //     let mut arr = vec![5,4,3,2,1];
    //     let sorted = vec![1,2,3,4,5];
    //     insertion_sort(&mut arr);

    //     assert_eq!(arr,sorted);
    // }
}
