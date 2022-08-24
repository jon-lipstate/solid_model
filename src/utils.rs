pub fn compare(a: f32, b: f32, tol: f32) -> i32 {
    let delta = f32::abs(a - b);
    if delta < tol {
        return 0;
    } else if a > b {
        return 1;
    } else {
        return -1;
    }
}
