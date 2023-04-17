pub fn get_string<T>(key: T) -> &'static str
where
    T: ToString,
{
    let binding = key.to_string();
    let value = super::get_value(&binding);
    let value = value.expect("value not found");
    let res = Box::leak(value.to_owned().into_boxed_str());
    res
}

pub fn set<T>(key: T, value: String) -> ()
where
    T: ToString,
{
    let binding = key.to_string();
    let key = Box::leak(binding.to_owned().into_boxed_str());
    let value = Box::leak(value.to_owned().into_boxed_str());
    super::set_value(key, value);
}