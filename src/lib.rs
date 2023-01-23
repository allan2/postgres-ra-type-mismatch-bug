pub fn type_mismatch_ex() {
    let mut params = Vec::<&(dyn Sync)>::with_capacity(2);
    let nums = vec![1, 2];
    for n in &nums {
        params.push(n);
    }
}