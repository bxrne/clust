use crate::config::Config;

pub trait ClusterClient {
    fn status(&self) -> String;
    fn get_pods(&self) -> Vec<String>;
    fn get_contexts(&self) -> Vec<String> {
        vec!["default".to_string(), "dev".to_string(), "prod".to_string()]
    }
}

pub struct SimulatedClient;

impl ClusterClient for SimulatedClient {
    fn status(&self) -> String {
        "Simulating cluster".to_string()
    }

    fn get_pods(&self) -> Vec<String> {
        vec![
            "pod-1".to_string(),
            "pod-2".to_string(),
            "pod-3".to_string(),
        ]
    }
}

pub struct RealClient {
    ctx: String,
}

impl RealClient {
    pub fn new(ctx: String) -> Self {
        Self { ctx }
    }
}

impl ClusterClient for RealClient {
    fn status(&self) -> String {
        format!("Real cluster client (stub) with context '{}'", self.ctx)
    }

    fn get_pods(&self) -> Vec<String> {
        vec!["POD LISTING NOT IMPLEMENTED".to_string()] // TODO:implement real pod listing
    }
}

pub fn get_client() -> Box<dyn ClusterClient> {
    let config = Config::load();
    if config.simulated {
        println!("Simulated cluster client initialized.");
        Box::new(SimulatedClient)
    } else {
        println!("Real cluster client initialized (stub).");
        Box::new(RealClient::new("default".to_string())) // TODO: implement real client
    }
}

#[cfg(test)]
mod tests {
    // Simulated client tests
    #[test]
    fn test_simulated_client_status() {
        let client = SimulatedClient;
        assert_eq!(client.status(), "Simulating cluster");
    }

    #[test]
    fn test_simulated_client_get_pods() {
        let client = SimulatedClient;
        let pods = client.get_pods();
        assert_eq!(pods.len(), 3);
        assert_eq!(pods[0], "pod-1");
    }
}
