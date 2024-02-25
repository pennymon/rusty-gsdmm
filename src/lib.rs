
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
                }
                doc_vector.push(word_index_map.get(word).unwrap().clone());
            }

            // dedupe vector and compact
            doc_vector.sort();
            doc_vector.dedup();
            doc_vector.shrink_to_fit();

            // stash
            doc_vectors.push(doc_vector);
        }
        let V = index as f64;
        println!("Fitting with alpha={}, beta={}, K={}, maxit={}, vocab size={}", alpha, beta, K, maxit, V as u32);

        let clusters = (0_usize..K).collect::<Vec<usize>>();
        let mut d_z: Vec<usize> = (0_usize..D).map(|_| 0_usize).collect::<Vec<usize>>(); // doc labels
        let mut m_z: Vec<u32> = GSDMM::zero_vector(K);  // cluster sizes
        let mut n_z: Vec<u32> = GSDMM::zero_vector(K);  // cluster word counts
        let mut n_z_w = Vec::<FnvHashMap<usize, u32>>::with_capacity(K);  // container for cluster word distributions
        for _ in 0_usize..K {
            let m = FnvHashMap::<usize, u32>::with_capacity_and_hasher(max(vocab.len() / 10, 100), Default::default());
            &n_z_w.push(m);
        }

        // randomly initialize cluster assignment
        let p = (0..K).map(|_| 1_f64 / (K as f64)).collect::<Vec<f64>>();

        let choices = random_choice().random_choice_f64(&clusters, &p, D) ;
        for i in 0..D {
            let z = choices[i].clone();
            let ref doc = doc_vectors[i];
            d_z[i] = z;
            m_z[z] += 1;
            n_z[z] += doc.len() as u32;
            let ref mut clust_words: FnvHashMap<usize, u32> = n_z_w[z];
            for word in doc {
                if !clust_words.contains_key(word) {
                    clust_words.insert(word.clone(), 0_u32);
                }
                    * clust_words.get_mut(word).unwrap() += 1_u32;
            }
        }

        GSDMM {
            alpha: alpha,
            beta: beta,
            K: K,
            V: V,
            D: D,
            maxit:maxit,
            doc_vectors:doc_vectors,
            clusters: clusters.clone(),     // Don't totally get why we need the clone here!
            labels: d_z,
            cluster_counts: m_z,
            cluster_word_counts: n_z,
            word_index_map: word_index_map,
            index_word_map: index_word_map,