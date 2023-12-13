fn main() {
    get_users();
}

fn get_users() {
    let url = "https://jsonplaceholder.typicode.com";

    let body = reqwest::blocking::get(format!("{url}/users"))
        .unwrap()
        .text()
        .unwrap();

    println!("{body}");
}
