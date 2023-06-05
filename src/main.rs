use std::env;
use std::fs::write;
use std::process::exit;

#[derive(Debug)]
pub struct ActionInputs {
    pub token: String,
    pub registry: Option<String>,
    pub name: Option<String>,
    pub strict: Option<bool>,
    pub config: Option<String>,
}

fn main() {
    if let Ok(output_path) = env::var("GITHUB_OUTPUT") {
        let inputs = get_inputs().expect("Failed to get inputs");
        deploy_wasmer_app(inputs).expect("Deployment failed");
        write(output_path, "I am so cool!").unwrap();
        exit(0);
    } else {
        println!("You need to set env variable GITHUB_OUTPUT");
        exit(1);
    }
}

fn get_inputs() -> Result<ActionInputs, &'static str> {
    let mut args = std::env::args().skip(1); // Skip the first argument (the program name)

    let token = args.next().ok_or("Input TOKEN is required")?;

    let registry = args.next();
    let name = args.next();
    let strict = args.next().and_then(|s| s.parse::<bool>().ok());
    let config = args.next();

    Ok(ActionInputs {
        token,
        registry,
        name,
        config,
        strict,
    })
}

fn deploy_wasmer_app(inputs: ActionInputs) -> Result<(), &'static str> {
    // Use the inputs to perform the deployment
    // You will have to implement the logic for deploying the app to wasmer.io using their GraphQL interface
    println!("{:?}", inputs);
    Ok(())
}
