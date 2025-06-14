

pub fn get_shader(name: String) -> String{
    let os = name.to_owned();
    let mut prefix = "C:/Users/kgil2/rust/minetest/resources/shaders".to_owned();
    prefix.push_str(&os);
    prefix
}