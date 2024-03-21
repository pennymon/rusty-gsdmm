
extern crate docopt;
extern crate rustc_serialize;
extern crate gsdmm;

use gsdmm::GSDMM;
use docopt::Docopt;
use std::io::{BufRead,BufReader};
use std::fs::File;
use std::collections::HashSet;
use std::io::Write;

const USAGE: &'static str ="
Gibbs sampling algorithm for a Dirichlet Mixture Model of Yin and Wang 2014.

Usage:
  gsdmm <datafile> <vocabfile> <outprefix> [-k <max_clusters>] [-a <alpha>] [-b <beta>] [-m <maxit>]
  gsdmm (-h | --help)
  gsdmm --version

Options:
  -h --help             Show this screen.
  --version             Show version.
  -k=<K>                Upper bound on the number of possible clusters. [default: 8]
  -a --alpha=<alpha>    Alpha controls the probability that a student will join a table that is currently empty
                        When alpha is 0, no one will join an empty table. [default: 0.1]
  -b --beta=<beta>      Beta controls the student's affinity for other students with similar interests. A low beta means
                        that students desire to sit with students of similar interests. A high beta means they are less
                        concerned with affinity and are more influenced by the popularity of a table. [default: 0.1]
  -m --maxit=<m>        Maximum number of iterations. [default: 30]

";

#[derive(Debug, RustcDecodable)]
struct Args {
    //    flag_mode: isize,
    arg_datafile: String,
    arg_vocabfile: String,
    arg_outprefix: String,
    flag_k: usize,
    flag_alpha: f64,
    flag_beta: f64,
    flag_maxit: isize
}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // get the data and vocabulary
    let vocab:HashSet<String> = lines_from_file(&args.arg_vocabfile).into_iter().collect();