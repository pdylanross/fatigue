use petname::petname;

pub fn rand_action_name() -> String {
    petname(2, "-")
}
