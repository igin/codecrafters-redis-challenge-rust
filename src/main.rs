mod nredis;

use nredis::server;

fn main() {
    let server_config = server::Config {
        address: "127.0.0.1:6379",
    };
    server::listen(&server_config);
}
