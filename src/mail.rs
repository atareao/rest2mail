pub struct Mail{
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl Mail{
    pub fn new(from: &str, to: &str, subject: &str, body: &str)->Self{
        Self{
            from: from.to_string(),
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        }
    }
}
