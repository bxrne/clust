pub trait ClusterClient {
    fn status(&self) -> String;
    fn get_pods(&self) -> Vec<String>;
}

pub struct SimulatedClient;

impl ClusterClient for SimulatedClient {
    fn status(&self) -> String {
        "Connected to simulated cluster".to_string()
    }
    fn get_pods(&self) -> Vec<String> {
        vec![
            "pod-1".to_string(),
            "pod-2".to_string(),
            "pod-3".to_string(),
        ]
    }
}

pub fn get_client() -> Box<dyn ClusterClient> {
    println!("Simulated cluster client initialized.");
    Box::new(SimulatedClient)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST: GIVEN a SimulatedClient WHEN status() is called THEN it returns the correct status string
    #[test]
    fn test_simulated_client_status() {
        let client = SimulatedClient;
        assert_eq!(client.status(), "Connected to simulated cluster");
    }

    // TEST: GIVEN a SimulatedClient WHEN get_pods() is called THEN it returns a list of pods
    #[test]
    fn test_simulated_client_get_pods() {
        let client = SimulatedClient;
        let pods = client.get_pods();
        assert_eq!(pods.len(), 3);
        assert!(pods.contains(&"pod-1".to_string()));
    }

    // TEST: GIVEN get_client() WHEN called THEN it returns a trait object with correct status
    #[test]
    fn test_get_client_returns_trait_object() {
        let client = get_client();
        assert_eq!(client.status(), "Connected to simulated cluster");
    }
}
