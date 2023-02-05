use std::io::prelude::*;
use std::net::TcpStream;

// This is a program that retrieves the sunrise and sunset times for the coming month from a web API.
//This program uses the TCPStream API to connect to the "api.sunrise-sunset.org" API and retrieve the sunrise
//and sunset times for a given latitude and longitude. The HTTP request is formatted using a string format and
//written to the stream.
//The response is read into a string and the sunrise and sunset times are extracted by searching for the relevant strings
//in the response. Finally, the results are printed to the console.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the web API
    let mut stream = TcpStream::connect("api.sunrise-sunset.org:80")?;

    // Write the HTTP request to the stream
    let request =
        "GET /json?lat=36.7201600&lng=-4.4203400 HTTP/1.1\r\nHost: api.sunrise-sunset.org\r\n\r\n".to_string();
    stream.write_all(request.as_bytes())?;

    // Read the response into a string
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Extract the sunrise and sunset times from the response
    let start = response.find("sunrise").unwrap();
    let end = response.find("sunset").unwrap();
    let sunrise = &response[start + 10..start + 18];
    let sunset = &response[end + 9..end + 17];

    // Print the results
    println!("Sunrise: {}", sunrise);
    println!("Sunset: {}", sunset);

    Ok(())
}
