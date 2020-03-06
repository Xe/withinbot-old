use xe621::client::Client;

#[test]
fn impl_std_error_trait() -> Result<(), Box<dyn std::error::Error>> {
    Client::new("MyProject/1.0 (by username on e621)")?;

    Ok(())
}
