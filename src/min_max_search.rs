fn min_max_search(buf: &[i32], max: &mut i32, min: &mut i32) {
    // base case 1 
    if buf.len() == 1 {
        *min = buf[0];
        *max = buf[0];
        return;
    }
    
    // base case 2
    if buf.len() == 2 {
        if buf[0] < buf[1] {
            *min = buf[0];
            *max = buf[1];
        } else {
            *max = buf[0];
            *min = buf[1];
        }
        return;
    }

    let mid = buf.len() / 2;
    let (l, r) = buf.split_at(mid);
    let (mut min_a, mut max_a) = (i32::MAX, i32::MIN);
    let (mut min_b, mut max_b) = (i32::MAX, i32::MIN);
    min_max_search(l, &mut max_a, &mut min_a);
    min_max_search(r, &mut max_b, &mut min_b);

    *max = i32::max(max_b, max_a);
    *min = i32::min(min_b, min_a);
}

#[cfg(test)]
mod test {
    use super::min_max_search;

    #[test]
    fn run_min_max() {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let buf = vec![6, 2, 3, 4, 21, 20, 28, 10];
        min_max_search(&buf, &mut max, &mut min);
        println!("min: {}, max: {}", min, max);
    }
}
