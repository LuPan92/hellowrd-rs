extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form,"root");
    router.post("/gcd", post_gcd, "gcd");
    println!("Serving on 3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest)
                    .set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };
    
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest)
                    .set_mut(format!("form data must contain 'n'"));
            return Ok(response);
        }
        Some(nums) => nums
    };
    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed){
            Err(_) => {
                response.set_mut(status::BadRequest)
                        .set_mut(format!("form data must contain only numbers"));
                return Ok(response);
            }
            Ok(num) => {numbers.push(num);}

        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    response.set_mut(status::Ok)
            .set_mut(mime!(Text/Html; Charset=Utf8))
            .set_mut(format!(" The greatest common divisor of the numbers {:?} is <b>{}</b>",numbers, d));
    Ok(response)
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok).set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <!DOCTYPE html>
    <html>
        <title>GCD Calculator</title>
        <body>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Calculate</button>
        </form>
        </body>
    </html>
    "#);
    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}