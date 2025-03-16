trait Device {
    fn get_mac_address(&self) -> &String;
    fn get_ip_address(&self) -> &String;
}

impl Device for Host {
    fn get_mac_address(&self) -> &String {
        &self.mac_address
    }

    fn get_ip_address(&self) -> &String {
        &self.ip_address
    }
}

impl Device for Router {
    fn get_mac_address(&self) -> &String {
        &self.mac_address
    }
    
    fn get_ip_address(&self) -> &String {
        &self.ip_address
    }
}