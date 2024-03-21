
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