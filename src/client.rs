use ureq::Request;

pub struct Client {
    agent: ureq::Agent,
    headers: Vec<(String, String)>,
}

impl Client {
    pub fn new(agent: ureq::Agent, headers: Vec<(String, String)>) -> Self {
        Self { agent, headers }
    }

    pub fn get(&self, path: &str) -> Request {
        let mut request = self.agent.get(path);
        for (key, value) in &self.headers {
            request = request.set(key, value)
        }
        request
    }
}
