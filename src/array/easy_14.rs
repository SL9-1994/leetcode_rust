pub fn easy_14() -> String {
    const STRS_1: [&str; 3] = ["flower", "flow", "flight"];
    const STRS_2: [&str; 3] = ["dog", "racecar", "car"];

    println!(
        "{}",
        longest_common_prefix(STRS_1.iter().map(|&s| s.to_string()).collect())
    );
    println!(
        "{}",
        longest_common_prefix(STRS_2.iter().map(|&s| s.to_string()).collect())
    );

    "easy_14".to_string()
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = strs[0].clone();

    for s in strs.iter().skip(1) {
        // バイト列を先頭から比較し，一致する長さを探す
        let end = prefix
            .as_bytes()
            .iter()
            .zip(s.as_bytes())
            .take_while(|(c1, c2)| c1 == c2)
            .count();

        // 一致したところまで削る
        prefix.truncate(end);
    }

    prefix
}
