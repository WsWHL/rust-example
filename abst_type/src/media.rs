// trait 创建一个特征
pub trait Playable {
    fn play(&self);
    fn pause() {
        println!("Paused");
    }
}