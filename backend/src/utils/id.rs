use nanoid::nanoid;

pub fn generate_id() -> String {
    return nanoid!(6);
}