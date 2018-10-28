use rand::Rng;

///Generates random number in range [left...right)
pub fn random_rng<T: rand::distributions::uniform::SampleUniform>(left: T, right: T) -> T {
    let mut rng = rand::thread_rng();

    rng.sample(rand::distributions::Uniform::new(left, right))
}
