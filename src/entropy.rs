
use std::collections::HashMap;
use std::f64;

use crate::bgdistribution;
use crate::options::BgDist;
//use crate::weighting::SYMBOL;

/* Amino acid symbol list without gap. */
static mut SYMBOL : Vec<char> = Vec::new();

pub struct JsDivergence {
	pub cons_capra07_list : Vec<f64>,
}

impl JsDivergence {
	pub fn new() -> JsDivergence {

		let cons_capra07_list : Vec<f64> = Vec::new();

		/* 20 amino acid symbols to calculate relative entropy, ignoring gaps. */
		unsafe { SYMBOL = "ARNDCQEGHILKMFPSTWYV".chars().collect(); }
		//println!( "{:?}", SYMBOL );

		JsDivergence {
			cons_capra07_list : cons_capra07_list,
		}
	}

	pub fn calc_js_divergence(
		&mut self,
		site_list    : &Vec<String>,
		weight_list  : &Vec<f64>,
		gap_pen_list : &Vec<f64>,
		bgdist       : &BgDist
	) {
		let num_site : usize = ( *site_list ).len();

		/* Define 'q' distribution ( background  distribution ). */
		let q : HashMap<char, f64> = bgdistribution::define_bg_dist( bgdist );

		self.cons_capra07_list = vec![ 0.0; num_site ];
		for i in 0 .. num_site {
			let js_dive : f64 = js_dive( &( *site_list )[ i ], weight_list, ( *gap_pen_list )[ i ], &q );
			( self.cons_capra07_list )[ i ] += js_dive;
		}

		( self.cons_capra07_list ).shrink_to_fit();

	}
}

fn js_dive(
	site_arg    : &String,
	weight_list : &Vec<f64>,
	gap_penalty : f64,
	q           : &HashMap<char, f64>
) -> f64 {

	let site : Vec<char> = ( *site_arg ).chars().collect();
	//println!( "site : {:?}", site );

	/* Make pc distribution ( site distribution ). */
	let mut pc : HashMap<char, f64> = weighted_freq_count( &site, weight_list );

	/* Modify site distributions to take account of gap ignoring. */
	unsafe {
		let mut sum_weight : f64 = 0.0;
		for aa in SYMBOL.iter() {
			sum_weight += pc[ aa ];
		}
		//println!( "Sum of weighting scores ignoring gaps( denominator of the probability ) : {:.3}", sum_weight );

		for aa in SYMBOL.iter() {
			pc.insert( *aa, pc[ aa ] / sum_weight );
		}
	}

	/*
	 * Make 'r' distribution.
	 * r  = ( pc + q ) / 2
	 * pc = site distribution
	 * q  = background distribution
	*/
	let mut r : HashMap<char, f64> = HashMap::new();
	unsafe {
		for aa in SYMBOL.iter() {
			r.insert( *aa, ( 0.5 * pc[ aa ] ) + ( 0.5 * ( *q )[ aa ] ) );
		}
	}

	/* Calculate Jensen-Shannon divergence. */
	let mut js_dive : f64 = 0.0;
	unsafe {
		for aa in SYMBOL.iter() {
			js_dive +=  ( pc[ aa ] * ( pc[ aa ] / r[ aa ] ).log2() ) + ( ( *q )[ aa ] * ( ( ( *q )[ aa ] / r[ aa ] ).log2() ) );
		}
	}

	js_dive = 0.5 * js_dive;

	/* Give the gap penalty */
	js_dive = js_dive * gap_penalty;

	//println!( "\nJS divergence : {:.3}\n", re );

	js_dive
}

fn weighted_freq_count( site : &Vec<char>, weight_list : &Vec<f64> ) -> HashMap<char, f64> {

	let len_site : usize = ( *site ).len();

	/* Define the pseudocount (10e-8). */
	let pseudo_count : f64 = 0.0000001;

	/* Define a hashmap to count AA frequency in a site. */
	let mut freq : HashMap<char, f64> = HashMap::new();
	unsafe {
		for aa in SYMBOL.iter() { freq.insert( *aa, pseudo_count ); }
	}
	//println!( "{:?}", freq );

	/*
	 * Count a frequency of each AA in a site taking accont of sequence weighting.
	 * Add a weighting score instead of simple increment (+1.0).
	 * !!! Gaps are ignored !!!
	 * aa               = One letter of AA in a site.
	 * add              = Weighting score add instead of 1.0.
	 * weight_list[ i ] = Weighting score of i th sequence.
	 * freq             = AA - weighted frequency hashmap.
	*/
	for i in 0 .. len_site {
		let aa  : char = ( *site )[ i ];
		if aa != '-' {
			let add : f64  = freq[ &aa ] + ( *weight_list )[ i ];
			freq.insert( aa, add );
		}
	}
	//println!( "Frequency : {:?}", freq );

	freq
}
