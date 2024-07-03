/// 给测试计时
fn timing(f: impl FnOnce(), des: &str) {
    use std::time::Instant;
    let start = Instant::now();
    let _ = f();
    println!("{} time: {:?}", des, start.elapsed());
}
