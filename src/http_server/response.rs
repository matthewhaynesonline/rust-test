// use std::fs;

pub fn get_http_response(request_line: &String) -> String {
    let (status_line, contents) = get_request_route(request_line);
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    return response;
}

pub fn get_request_route(request_line: &String) -> (&str, &str) {
    // TODO: concat str at compile time without heap?
    // const PROTOCOL: &str = "HTTP/1.1";

    // use include_str macro to get compile time checks vs
    // fs::read which is runtime
    let templates = [
        include_str!("./templates/index.html"),
        include_str!("./templates/404.html"),
    ];

    // let contents = fs::read_to_string(format!("./templates/{template_filename}")).unwrap();

    let route = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", templates[0]),
        _ => ("HTTP/1.1 404 NOT FOUND", templates[1]),
    };

    return route;
}
