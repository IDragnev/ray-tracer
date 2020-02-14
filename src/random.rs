use rand::Rng;

pub fn random_float_from_0_to_1() -> f32 {
    gen_range(0.0, 1.0)
}

pub fn gen_range<T>(from: T, to: T) -> T 
    where T: rand::distributions::uniform::SampleUniform {
    rand::thread_rng().gen_range(from, to)
}