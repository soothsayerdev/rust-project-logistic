pub fn calculate_route(address_origin: &str, address_destination: &str) -> String {
    format!(
        "Optimized route de '{}' até '{}'",
        address_origin, address_destination
    )
}