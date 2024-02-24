
#![allow(non_snake_case)]

extern crate random_choice;
extern crate fnv;

use std::collections::{HashSet, HashMap};
use std::cmp::max;
use fnv::FnvHashMap;
use self::random_choice::random_choice;

pub struct GSDMM {
    alpha: f64,
    beta: f64,
    K:usize,
    V:f64,
    D:usize,
    maxit:isize,
    clusters: Vec<usize>,
    pub doc_vectors:Vec<Vec<usize>>,
    pub labels: Vec<usize>,
    pub cluster_counts: Vec<u32>,
    pub cluster_word_counts:Vec<u32>,
    pub word_index_map:HashMap<String, usize>,
    pub index_word_map:HashMap<usize, String>,
    pub cluster_word_distributions: Vec<FnvHashMap<usize,u32>>
}

impl GSDMM {
    pub fn new(alpha:f64, beta:f64, K: usize, maxit:isize, vocab:HashSet<String>, docs:Vec<Vec<String>>) -> GSDMM {
        let D = docs.len();

        // compute utilized vocabulary size.
        let mut word_index_map = HashMap::<String, usize>::with_capacity(vocab.len()/2);
        let mut index_word_map = HashMap::<usize, String>::with_capacity(vocab.len()/2);
        let mut index = 0_usize;
        let mut doc_vectors = Vec::<Vec<usize>>::with_capacity(D);
        for doc in &docs {
            let mut doc_vector = Vec::<usize>::with_capacity(doc.len());
            for word in doc {
                if !word_index_map.contains_key(word) {
                    word_index_map.insert(word.clone(), index);
                    index_word_map.insert(index, word.clone());
                    index+=1;