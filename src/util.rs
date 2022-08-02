use rand;

pub fn pick_random_element_from_vec<T>(vec: &Vec<T>) -> &T {
    let a_t = rand::random::<usize>() % vec.len();
    &vec[a_t]
}
