// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::{get, web, HttpResponse, Responder};

use std::collections::HashSet;

use lazy_static::lazy_static;
// use rocket::serde::json::Json;
use semsimian::termset_pairwise_similarity::TermsetPairwiseSimilarity as Tsps;
use semsimian::{RustSemsimian, TermID};
use utils::get_rss_instance;

pub mod utils;

lazy_static! {
    static ref RSS: RustSemsimian = get_rss_instance();
}

//--- ROUTES ---//
#[get("/")]
pub async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Semsimian Server Online")
}

#[get("/compare/{termset1}/{termset2}")]
pub async fn compare_termsets(termset1: String, termset2: String) -> impl Responder {
    // split termset1 and termset2 into vectors of TermIDs
    let mut terms1: HashSet<TermID> = HashSet::new();
    for term in termset1.split(",") {
        terms1.insert(term.to_string());
    }
    let mut terms2: HashSet<TermID> = HashSet::new();
    for term in termset2.split(",") {
        terms2.insert(term.to_string());
    }
    // info!(
    println!(
        "Comparing termsets:\
        \n\tTermset 1: {:?}\
        \n\tTermset 2: {:?}\
        \n",
        terms1, terms2
    );
    let result = RSS.termset_pairwise_similarity(&terms1, &terms2);
    HttpResponse::Ok().json(result)
}
