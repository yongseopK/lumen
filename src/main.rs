mod http;
mod html;
mod dom;
mod css;
mod style;
mod layout;
mod paint;
mod renderer;

fn main() {
    let response =http::fetch("http://www.example.com/");

    println!("{:?}", response);
}

