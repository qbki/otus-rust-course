const ROOM_NAME: &str = "Some New Room";
const OUTLET_NAME: &str = "Some New Outlet";
const THERMOMETER_NAME: &str = "Some New Thermometer";

type RequestResult = Result<(), Box<dyn std::error::Error>>;

fn url(path: &Vec<&str>) -> String {
    format!("http:\\/\\/localhost:8000\\/{}", path.join("/"))
}

fn print_state(path: &Vec<&str>) -> RequestResult {
    let response = reqwest::blocking::get(url(path))?.text()?;
    println!("{}\n\n", response);
    Ok(())
}

fn post_request(path: &Vec<&str>, payload: &Vec<(&str, &str)>) -> RequestResult {
    let client = reqwest::blocking::Client::new();
    client.post(url(path)).form(payload).send()?;
    Ok(())
}

fn delete_request(path: &Vec<&str>) -> RequestResult {
    let client = reqwest::blocking::Client::new();
    client.delete(url(path)).send()?;
    Ok(())
}

fn main() -> RequestResult {
    println!("Initial state:");
    print_state(&vec![])?;

    println!("Added a new room:");
    post_request(&vec!["room"], &vec![("name", ROOM_NAME)])?;
    print_state(&vec![])?;

    println!("Added a new thermometer and outlet:");
    post_request(
        &vec!["room", ROOM_NAME, "device"],
        &vec![("name", THERMOMETER_NAME), ("device_type", "THERMOMETER")],
    )?;
    post_request(
        &vec!["room", ROOM_NAME, "device"],
        &vec![("name", OUTLET_NAME), ("device_type", "OUTLET")],
    )?;
    print_state(&vec![])?;

    println!("Shown a room:");
    print_state(&vec!["room", ROOM_NAME])?;

    println!("Shown a device:");
    print_state(&vec!["room", ROOM_NAME, "device", OUTLET_NAME])?;

    println!("Removed a device");
    delete_request(&vec!["room", ROOM_NAME, "device", OUTLET_NAME])?;
    print_state(&vec![])?;

    println!("Removed a room");
    delete_request(&vec!["room", ROOM_NAME])?;
    print_state(&vec![])?;

    Ok(())
}
