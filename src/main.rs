
use std::time::Instant;
use colored::*;

mod bgdistribution;
mod entropy;
mod error;
mod fasta;
mod gap;
mod options;
mod result;
mod weighting;

fn main()
{
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

	let weight_list : Vec<f64> = weighting::seq_weight
	(
		&( data.seq_list  ),
		&( data.site_list ),
		&( opts.weight    )
	);

	/*
	println!( "\nSequence weighting :\n" );
	for i in 0 .. weight_list.len() {
		println!( "Weight of Sequence {} : {}", i + 1, weight_list[ i ] );
	}
	*/

	/* Calculate gap penalties taking acconut of sequence weighting. */
	let gap_pen_list : Vec<f64> = gap::weight_gap_penalty( &( data.site_list ), &weight_list );

	/*
	for i in 0 .. gap_pen_list.len() {
		println!( "Gap penalty of site {} : {:.4}", i + 1, gap_pen_list[ i ] );
	}
	*/

	let cons_capra07_list : Vec<f64> = entropy::js_divergence
	( 
		&( data.site_list ),
		&weight_list,
		&gap_pen_list,
		&( opts.bgdist )
	);

	/*
	for i in 0 .. cons_capra07_list.len() {
		println!( "Jensen-Shannon divergence site {} : {:.3}", i + 1, cons_capra07_list[ i ] );
	}
	*/

	/* Show result */
	result::show_result
	(
		&( data.site_list ),
		&cons_capra07_list,
		&( opts.colorize  )
	);

	/* Save result */
	result::save_result
	(
		&( data.site_list ),
		&cons_capra07_list,
		&( opts.output    )
	);

	println!( "{}", "\nProgram completed !!!\n".green() );

	/* Elapsed time : End */
	let end = start.elapsed();
	println!( "Total elapsed time : {:?}", end );
}
