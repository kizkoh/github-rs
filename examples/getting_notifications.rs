extern crate github_rs;
use github_rs::client::Github;

fn main() {
    let client = Github::new("");
    let notifications = client.get().notifications().execute();
    match notifications {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json {
                println!("{}", json);
            }
        }
        Err(e) => println!("{}", e),
    }
}
