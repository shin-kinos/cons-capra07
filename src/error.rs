
use std::process;
//use colored::*;

pub fn error_bomb( arg : &str ) {

	println!( "{}", "\n\x1b[31;1m!!! ERROR !!!\x1b[0m\n" );

	match arg {
		"seq_title_not_same"    => println!( "Inadequate format in Multi-FASTA file." ),
		"seq_len_not_same"      => println!( "The length of all the sequences must be same." ),
		"site_ent_len_not_same" => println!( "Length of ( *site_list ) != Length of ( *cons_re_list )" ), 
		"non_standard_residue"  => println!( "Non-standard residue was observed in the input file." ),
		"unexpected_symbol"     => println!( "Unexpected symbol was observed in the input file." ),
		_                       => (),
	}

	println!( "{}", "\n\x1b[31;1m!!! Program halted !!!\x1b[0m\n" );

	process::exit( 1 );
}
