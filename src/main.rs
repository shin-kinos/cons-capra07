
use std::time::Instant;
//use colored::*;

mod bgdistribution;
mod entropy;
mod error;
mod fasta;
mod gap;
mod options;
mod result;
mod weighting;
mod window;

fn main() {

	println!( "\nCalculating conservation score a site in MSA by Jensen-Shannon divergence.\n" );
	println!( "Capra, John A., and Mona Singh. \"Predicting functionally important residues from sequence conservation.\" Bioinformatics 23.15 (2007)." );

	/* Elapsed time : Start */
	let start = Instant::now();

	/* Set options. */
	let opts = options::Options::new();
	opts.show_parameter();

	/* Read an input file and get FASTA information. */
	let mut data = fasta::Fasta::new();
	data.read_fasta_info( &( opts.input ) );

	/* Check whether the input file is correct FASTA format. */
	data.check_fasta_info( &( opts.tolerate ) );

	/* Get site information as Vec<String>. */
	data.get_site_list();

	/* Check window size for moving average. */
	window::check_window_size( opts.window, ( data.site_list ).len() );

	/*
	println!( "\nInputfile content :\n" );
	for i in 0 .. ( data.seq_list ).len() {
		println!( "Title    {} : {}", i + 1, ( data.title_list )[ i ] );
		println!( "Sequence {} : {}", i + 1, ( data.seq_list )[ i ] );
	}
	*/

	/*
	println!( "\nSite content :\n" );
	for i in 0 .. ( data.site_list ).len() {
		println!( "Site {} : {}", i + 1, ( data.site_list )[ i ] );
	}
	*/

	/* Sequence weighting. */
	let mut weighting = weighting::SequenceWeighting::new();
	weighting.calc_weight_list(
		&( data.seq_list  ),
		&( data.site_list ),
		&( opts.weight    )
	);

	weighting.show_sum_weight();

	/*
	println!( "\nSequence weighting :\n" );
	for i in 0 .. ( weighting.weight_list ).len() {
		println!( "Weight of Sequence {} : {}", i + 1, ( weighting.weight_list )[ i ] );
	}
	*/

	/* Calculate gap penalties taking acconut of sequence weighting. */
	let mut gap = gap::GapPenalty::new();
	gap.calc_gap_penalty( &( data.site_list ), &( weighting.weight_list ) );

	/*
	for i in 0 .. ( gap.gap_pen_list ).len() {
		println!( "Gap penalty of site {} : {:.4}", i + 1, ( gap.gap_pen_list )[ i ] );
	}
	*/

	/* Calculate J.S. Divergence */
	let mut entropy = entropy::JsDivergence::new();
	entropy.calc_js_divergence(
		&( data.site_list ),
		&( weighting.weight_list ),
		&( gap.gap_pen_list ),
		&( opts.bgdist )
	);

	/*
	for i in 0 .. ( entropy.cons_capra07_list ).len() {
		println!( "Jensen-Shannon divergence site {} : {:.5}", i + 1, ( entropy.cons_capra07_list )[ i ] );
	}
	*/


	if ( opts.window ) != 0 { entropy.cons_capra07_list = window::moving_average( &( entropy.cons_capra07_list ), opts.window ); }

	/*
	for i in 0 .. ( entropy.cons_capra07_list ).len() {
		println!( "W-JS divergence site {} : {:.5}", i + 1, ( entropy.cons_capra07_list )[ i ] );
	}
	*/


	/* Show result */
	result::show_result(
		&( data.site_list ),
		&( entropy.cons_capra07_list ),
		&( opts.colorize )
	);

	/* Save result */
	result::save_result(
		&( data.site_list ),
		&( entropy.cons_capra07_list),
		&( opts.output )
	);

	println!( "{}", "\n\x1b[32;1mProgram completed !!!\x1b[0m\n" );

	/* Elapsed time : End */
	let end = start.elapsed();
	println!( "Total elapsed time : {:?}", end );
}
