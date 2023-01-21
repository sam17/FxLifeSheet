pub trait BaseIntegration {
    fn name(&self) -> String;
    fn authorize(&self) -> bool;
    fn get_data(&self, start_date: String, end_date: String) -> String;
}

pub struct Oura {
    name: String
}

impl Oura {
    pub fn new() -> Oura {
        return Oura {
            name: "Oura".to_string()
        };
    }
}

impl BaseIntegration for Oura  {

    fn name(&self) -> String {
        return "Oura".to_string();
    }

    fn authorize(&self) -> bool {
        return true;
    }

    fn get_data(&self, start_date: String, end_date: String) -> String {
        return "Oura data".to_string();
    }
}
