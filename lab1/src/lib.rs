pub mod insertion_merge;

#[cfg(test)]
#[allow(unused_imports)]
pub mod test {
    use std::{
        fs::File,
        io::{Read, Write},
    };

    use sc2001::Estimates;

 
    #[test]
    #[ignore]
    fn export_all_data_to_csv() {
        let array_sz = ["1k", "10k", "100k", "500k", "1mill"];

        for name in array_sz {
            let mut all_estimates = String::from("sz,mean,median\n");
            for sz in 3..=128 {
                let mut buf = String::new();
                let file_name = format!(
                    "target/criterion/insertion_merge_sort({name}_s{sz})/new/estimates.json"
                );

                println!("{file_name}");

                let mut f = File::open(file_name).unwrap();
                let _ = f.read_to_string(&mut buf).unwrap();

                let estimates = serde_json::from_str::<Estimates>(&buf).unwrap();
                let (mean, median) = (
                    estimates.mean.point_estimate / 1000000.0,
                    estimates.median.point_estimate / 1000000.0,
                );

                all_estimates.push_str(&format!("{sz},{mean},{median}\n",));
            }

            let mut f = File::create(format!("bench_{name}.csv")).unwrap();
            f.write_all(all_estimates.as_bytes()).unwrap();
        }
    }
}
