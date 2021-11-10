use hecs::World;

pub trait GameMode {
    fn initalize(world: &mut World);
    fn destroy(world: &mut World);
    fn is_over() -> bool;
}
