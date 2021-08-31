use std::collections::HashMap;

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


fn calc_distance(window: Vec<&str>) -> usize{
    let mut res: usize = 1;
    let mut last = window.first().unwrap();
    for val in window.iter(){
        if val == last{
            continue;
        }
        else{
            res = res + 1;
            last = val;
        }
    }
    res
}

pub fn reuse_distance(trace: Vec<&str>)-> HashMap<usize, f32> {
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
    dist_map
    /*
        End function
    */
}






#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn smoke() {
        assert!(1 == 1);
    }

    #[test]
    fn ri_cyclic() {
        let arr = vec!["a", "b", "c", "a", "b", "c"];
        let dist = reuse_interval(arr);

        let mut map = HashMap::new();
        map.insert(3, 1.0);

        assert_eq!(dist, map);
    }

    #[test]
    fn ri_sawtooth() {
        let arr = vec!["a", "b", "c", "c", "b", "a"];
        let dist = reuse_interval(arr);

        let mut map = HashMap::new();
        map.insert(1, 0.33333334);
        map.insert(3, 0.33333334);
        map.insert(5, 0.33333334);

        assert_eq!(dist, map);
    }

    // #[test]
    // fn insert_sort() {
    //     let mut arr = vec![5,4,3,2,1];
    //     let sorted = vec![1,2,3,4,5];
    //     insertion_sort(&mut arr);

    //     assert_eq!(arr,sorted);
    // }
}
