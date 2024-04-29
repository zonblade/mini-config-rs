pub fn get_string<T>(key: T) -> &'static str
where
    T: ToString,
{
    let binding = key.to_string();
    let value = super::get_value(&binding);
    std::mem::drop(binding);
    let value = value.unwrap_or("undefined");
    value
}

pub fn get_str(key: &str) -> &'static str {
    let value = super::get_value(key);
    let value = value.unwrap_or("undefined");
    value
}

pub fn set<T>(key: T, value: &str) -> ()
where
    T: ToString,
{
    let key_ref = Box::leak(Box::new(key.to_string()));
    let value_ref = Box::leak(Box::new(value.to_string()));
    super::set_value(key_ref, value_ref);
}