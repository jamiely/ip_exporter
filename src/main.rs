#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate prometheus;
#[macro_use] extern crate lazy_static;

extern crate pnet;
use std::env;
use prometheus::{TextEncoder, Encoder, IntGaugeVec};
use pnet::datalink;
use std::collections::{HashSet};
use rocket::State;
use rocket::response::content::Html;

lazy_static! {
    static ref IP_GAUGE: IntGaugeVec = register_int_gauge_vec!(
        "ip_is_bound",
        "Whether or not the IP is bound. 1 is the IP is bound and 0 if not.",
        &["ip"]
    )
    .unwrap();
}

struct Settings {
    ips: HashSet<String>
}

#[get("/metrics")]
fn metrics(settings: State<Settings>) -> String {
    let current_ips = datalink::interfaces().iter()
        .flat_map(|iface| iface.ips.iter())
        .map(|ip| ip.ip().to_string()).collect::<HashSet<_>>();

    for ip in &settings.ips {
        if current_ips.contains(ip) {
            IP_GAUGE.with_label_values(&[ip]).set(1);
        } else {
            IP_GAUGE.with_label_values(&[ip]).set(0);
        }
    }

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r"<html><a href='/metrics'>metrics here.</a></html>")
}

fn main() {
    let mut check_ips = HashSet::new();
    let ip_list = env::var("IP_LIST").unwrap_or("127.0.0.1".to_owned());
    for ip in ip_list.split_ascii_whitespace() {
        check_ips.insert(ip.to_owned());
    }

    rocket::ignite()
        .manage(Settings { ips: check_ips })
        .mount("/", routes![index, metrics]).launch();
}
