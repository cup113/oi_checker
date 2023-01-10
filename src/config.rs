mod cf_parsing;
mod cla_parsing;

pub fn get_config() {
    cla_parsing::parse_cla();
}
