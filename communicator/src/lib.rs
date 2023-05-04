pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        // ::client::connect() 동작하지 않음
        client::connect();
    }
}
