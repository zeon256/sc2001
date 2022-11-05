pub fn lcs_bottom_up(x: &str, y: &str) -> usize {
    let (n, m) = (x.len(), y.len());
    let mut buf = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if x.as_bytes()[i - 1] == y.as_bytes()[j - 1] {
                buf[i][j] = buf[i - 1][j - 1] + 1;
            } else if buf[i - 1][j] > buf[i][j - 1] {
                buf[i][j] = buf[i - 1][j];
            } else {
                buf[i][j] = buf[i][j - 1];
            }
        }
    }

    buf[n][m]
}

#[cfg(test)]
mod tests {
    use super::lcs_bottom_up;


    #[test]
    fn run_lcs() {
        let s = "ACGGA";
        let s1= "ACTG";
        assert_eq!(lcs_bottom_up(s, s1), 3);
        let s = "AGGTAB";
        let s1= "GXTXAYB";
        assert_eq!(lcs_bottom_up(s, s1), 4);
    }
}