use reqwest::blocking::Client;

fn main() {
    get_users();
    store_user();
    update_user();
}

fn get_users() {
    let url = "https://jsonplaceholder.typicode.com";

    let client = Client::new();
    let res = client.get(format!("{url}/users")).send().unwrap();

    let body = res.text().unwrap();

    println!("\n\n################## Get all users ##################");
    println!("{body}");
}

fn store_user() {
    let url = "https://jsonplaceholder.typicode.com";

    let new_user = r#"{
        "name": "Leanne Graham",
        "username": "Bret",
        "email": "Sincere@april.biz",
        "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
                "lat": "-37.3159",
                "lng": "81.1496"
            }
        },
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "company": {
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets"
        }
    }
  "#;

    let client = Client::new();
    let res = client
        .post(format!("{url}/users"))
        .body(new_user)
        .header("Content-type", "application/json; charset=UTF-8")
        .send()
        .unwrap();

    let body = res.text().unwrap();

    println!("\n\n################## Store user ##################");
    println!("{body}");
}

fn update_user() {
    let url = "https://jsonplaceholder.typicode.com";

    let to_update = r#"{"name": "Foo"}"#;

    let client = Client::new();
    let res = client
        .patch(format!("{url}/users/1"))
        .body(to_update)
        .header("Content-type", "application/json; charset=UTF-8")
        .send()
        .unwrap();

    let body = res.text().unwrap();

    println!("\n\n################## Update user ##################");
    println!("{body}");
}
