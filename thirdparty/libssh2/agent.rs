fn main() {
    // Almost all APIs require a `Session` to be available
    let session = ssh2::Session::new().unwrap();
    let mut agent = session.agent().unwrap();

    // Connect the agent and request a list of identities
    agent.connect().unwrap();
    agent.list_identities().unwrap();

    for identity in agent.identities().unwrap() {
        println!("{}", identity.comment());
        let pubkey = identity.blob();
        println!("{:?}", pubkey);
    }

    agent.disconnect().unwrap();
}
