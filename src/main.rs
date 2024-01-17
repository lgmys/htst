use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use colored::*;
use reqwest::{blocking::Response, Method};
use serde::Deserialize;
use tera::{Context as TeraContext, Tera};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    file: String,

    #[arg(short, long, default_value_t = 3)]
    timeout: u64,
}

#[derive(Debug)]
struct Context {
    args: Args,
}

#[derive(Deserialize, Debug)]
struct RequestDescription {
    url: String,
}

impl RequestDescription {
    fn try_from_file(template_path: &str) -> Result<Self> {
        let mut tera = Tera::default();
        tera.add_template_file(template_path, None)?;

        let raw_request_config = tera.render(template_path, &TeraContext::new())?;

        let request_description: RequestDescription = toml::from_str(&raw_request_config)?;

        Ok(request_description)
    }
}

fn run_request(ctx: &Context, request_description: &RequestDescription) -> Result<Response> {
    let client = reqwest::blocking::Client::new();
    let client = client
        .request(Method::GET, &request_description.url)
        .timeout(Duration::from_secs(ctx.args.timeout));

    let resp = client.send()?;

    Ok(resp)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let ctx = Context { args };

    let request_description = RequestDescription::try_from_file(&ctx.args.file)?;

    println!("METHOD {}", request_description.url.blue());
    println!("Running...");
    let resp = run_request(&ctx, &request_description)?;

    println!("Status: {}\n", resp.status());
    println!("Headers:");

    resp.headers().iter().for_each(|(name, value)| {
        println!("{}: {:?}", name, value);
    });

    println!("\n\nResponse text:");

    print!("{}", resp.text()?);

    Ok(())
}
