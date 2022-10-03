pub mod insertion_merge;

#[cfg(test)]
#[allow(unused_imports)]
pub mod test {
    use std::{
        fs::File,
        io::{Read, Write},
    };

    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize, Debug)]
    struct Estimates {
        mean: EstimateData,
        median: EstimateData,
        median_abs_dev: EstimateData,
        std_dev: EstimateData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct EstimateData {
        confidence_interval: ConfidenceInterval,
        point_estimate: f64,
        standard_error: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ConfidenceInterval {
        confidence_level: f64,
        lower_bound: f64,
        upper_bound: f64,
    }
 
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
