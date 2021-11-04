
use std::collections::HashMap;
use std::f64;

/*
 * Symbol frequency.
 * This variable is also used in "entropy::relative_entropy()".
 */
pub static mut SYMBOL : Vec<char> = Vec::new();

pub fn seq_weight( seq_list : &Vec<String>, site_list : &Vec<String>, arg_w : &String ) -> Vec<f64>
{
	/* Amino acid list for Position-Based mothod. */
	unsafe { SYMBOL = "ARNDCQEGHILKMFPSTWYV-".chars().collect(); }

	if *arg_w == "va"  { weight_va( seq_list ) }
	else               { weight_henikoff( site_list ) }

}

///////////////////////////////////////////////////////////////////////////
// POSITION-BASED METHOD
///////////////////////////////////////////////////////////////////////////

fn weight_henikoff( site_list : &Vec<String> /*, arg_t : &String */ ) -> Vec<f64>
{

	/* Number of the sequences and sites. */
	let num_seq  : usize = ( *site_list )[ 0 ].len();
	let num_site : usize = ( *site_list ).len();

	let mut weight_list : Vec<f64> = vec![ 0.0; num_seq ];

	/*
	 * Calculate weighting factor using position based method (Henikoff-Henikoff, 1994).
	 * r             = Number of AA types in a site.
	 * s             = Frequency of the AA in a site.
	 * weight_factor = 1 / (r * s).
	*/
	for site in site_list.iter() {
		//println!( "{}", *site );
		let r : usize = count_types( site );
		for i in 0 .. ( *site ).len() {
			let aa_vec : Vec<char> = ( *site ).chars().collect();
			let aa : char = aa_vec[ i ];
			//println!( "{}", aa );
			let s : usize = count_freq( aa, site );
			let weight_factor : f64 = 1.0 / ( ( r as f64 ) * ( s as f64 ) );
			//println!( "weight_factor : {}", weight_factor );
			weight_list[ i ] += weight_factor;
		}
	}

	let mut sum_weight : f64 = 0.0;

	/*
	 * Get sequence weight by calculating mean of weighting factors in each sites.
	 * num_site = Denominator of mean.
	*/
	for i in 0 .. weight_list.len() {
		weight_list[ i ] = weight_list[ i ] / ( num_site as f64 );
		//println!( "Weight of Sequence {} : {:.3}", i + 1, weight_list[ i ] );
		sum_weight += weight_list[ i ];
	}

	println!( "\nSum of sequence weighting : {:.3}", sum_weight );

	weight_list.shrink_to_fit();

	weight_list
}

fn count_types( arg_site : &String ) -> usize
{
	let mut count : HashMap<char, usize> = HashMap::new();

	unsafe {
		for aa in SYMBOL.iter() { count.insert( *aa, 0 ); }
	}

	for aa in ( *arg_site ).chars() {
		let inc : usize = count[ &aa ] + 1;
		count.insert( aa, inc );
	}

	let mut num_type : usize = 0;
	unsafe {
		for aa in SYMBOL.iter() {
			if count[ aa ] != 0 {
				num_type += 1;
			}
		}
	}

	//println!( "Number of AA types in {} : {}", *arg_site, num_types);

	num_type
}

fn count_freq( arg_aa : char, arg_site : &String ) -> usize
{
	let aa_list : Vec<char> = ( *arg_site ).chars().collect();
	let mut freq : usize = 0;

	for i in 0 .. aa_list.len() {
		if arg_aa == aa_list[ i ] {
			freq += 1;
		}
	}

	//println!( "Frequency of {} in {} : {}", arg_aa, *arg_site, freq );

	freq
}

///////////////////////////////////////////////////////////////////////////
// DISTANCE-BASED METHOD
///////////////////////////////////////////////////////////////////////////

fn weight_va( seq_list : &Vec<String> ) -> Vec<f64>
{

	/* Number of the sequences and sites. */
	let num_seq  : usize = ( *seq_list ).len();

	let mut weight_list : Vec<f64> = vec![ 0.0; num_seq ];

	/*
	 * Calculate pairwise distance by counting differd symbols (Vingron-Argos, 1989).
	 * seq_pair_1 = One pairwised sequence.
	 * seq_pair_2 = The other pairwised one.
	 * num_diff   = Number of the differences in pairwised sequences.
	*/
	for i in 0 .. num_seq {
		let seq_pair_1 : &String = &( seq_list[ i ] );
		//println!( "seq_pair_1 : {}", seq_pair_1 );
		for j in 0 .. num_seq {
			if i != j {
				let seq_pair_2 : &String = &( seq_list[ j ] );
				//println!( "seq_pair_2 : {}", seq_pair_2 );
				let num_diff : usize = count_diff( seq_pair_1, seq_pair_2 );
				weight_list[ i ] += num_diff as f64;
			}
		}
		//println!( "" );
	}

	//println!( "Weights : {:?}", weight_list );

	/* Normalize the weight factors so that sum is 1. */
	weight_list = normalize( &weight_list );
	//println!( "Normalized weights : {:?}", weight_list );

	let sum_norm_weight : f64 = ( weight_list ).iter().sum();
	println!( "\nSum of sequence weighting : {:.3}", sum_norm_weight );

	weight_list.shrink_to_fit();

	weight_list
}

fn count_diff( seq_1 : &String, seq_2 : &String ) -> usize
{
	let num_seq : usize = ( *seq_1 ).len();

	let seq_1_vec : Vec<char> = ( *seq_1 ).chars().collect();
	let seq_2_vec : Vec<char> = ( *seq_2 ).chars().collect();

	/*
	 * Count the number of different sybols.
	 * seq_1_vec = One pairwised sequence.
	 * seq_2_vec = The other pairwised one.
	 * counter   = Counter.
	*/
	let mut counter : usize = 0;
	for i in 0 .. num_seq {
		if seq_1_vec[ i ] != seq_2_vec[ i ] {
			counter += 1;
		}
	}

	counter
}

fn normalize( diff_list : &Vec<f64> ) -> Vec<f64>
{
	let len_list : usize = ( *diff_list ).len();

	let mut weight_norm : Vec<f64> = Vec::new();

	let sum : f64 = ( *diff_list ).iter().sum();

	/*
	 * Normalize the weight factors so that sum is 1.
	 * val_norm  = Normalized values.
	 * diff_list = Numerator (weights). 
	 * sum       = Denominator (Sum of weights).
	*/
	for i in 0 .. len_list {
		let val_norm : f64 = ( *diff_list )[ i ] / sum;
		weight_norm.push( val_norm );
	}

	weight_norm
}
