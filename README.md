
# Rusty-GSDMM: Short Text Clustering in Rust

The project brings you a quick and competent implementation of the Gibbs sampling algorithm for a Dirichlet Mixture Model, as proposed by [Yin and Wang 2014](https://pdfs.semanticscholar.org/058a/d0815ce350f0e7538e00868c762be78fe5ef.pdf), for short text clustering. Some highlights of the algorithm:
 - An upper limit `K` on the number of clusters is all that's required
 - Fast model convergence with the right parameter selection
 - Space-savvy and scalable

Courtesy to pennymon, we have an efficient GSDMM module in Rust. Refer to the [original project](https://github.com/rwalk/gsdmm) by rwalk for a simpler Python implementation.

## Movie Group Process: A Conceptual Model for GSDMM

As an effort to explain GSDMM with a better insight from the authors, the Movie Group Process was introduced.

Picture a professor conducting a film class with students assigned to `K` tables at the beginning. Each of these students have a list of their favorite films. The professor then repeatedly goes through the class register and each time a student's name is called out, they have to pick a new table satisfying one or both of the conditions:

- The new table is either vacant or has more students than the current table.
- The new table's students share similar film preferences.

Maintaining these rules should eventually lead the students to an optimal table configuration.

## Usage

The Rusty-GSDMM package includes a library that can be integrated into projects, and a standalone command line executable to carry out the GSDMM process on your data.

To use the executable, you first need to build it:
```shell
cargo build --release
```
For more details look at the help section:
```shell
./target/release/gsdmm -h
```
The trained executable can be deployed like this:
````
gsdmm <datafile> <vocabfile> <outprefix> [-k <max_clusters>] [-a <alpha>] [-b <beta>] [-m <maxit>]
````
...

## Practical Tips

Here are some practical tips when working with Rusty-GSDMM:

- The value of `K` should generally be in the same order of magnitude as the expected number of clusters. If the number of clusters remains constant across all iterations, `K` may need to be increased. Note that larger `K` values may increase computation time.

- The `alpha` and `beta` parameters need tuning for each dataset and use case. They tend to influence convergence behavior in opposite directions. Starting with a small subsample of documents may prove helpful in getting ballpark estimates for these parameters.

- Keep an eye on the number of clusters and the number of documents transferred in each iteration. Both should rapidly decrease and then stabilize. Neither number should increase significantly in subsequent iterations.