
use std::process;
//use colored::*;

pub enum ErrorType {
	SeqTitleNotSame,
	SeqLenNotSame,
	SiteEntLenNotSame,
	NonStandardResidue,
	UnexpectedSymbol,
}

pub fn error_bomb( error_type : ErrorType ) {

	println!( "{}", "\n\x1b[31;1m!!! ERROR !!!\x1b[0m\n" );

	match error_type {
		ErrorType::SeqTitleNotSame    => println!( "Inadequate format in Multi-FASTA file." ),
		ErrorType::SeqLenNotSame      => println!( "The length of all the sequences must be same." ),
		ErrorType::SiteEntLenNotSame  => println!( "Length of ( *site_list ) != Length of ( *cons_re_list )" ),
		ErrorType::NonStandardResidue => println!( "Non-standard residue was observed in the input file." ),
		ErrorType::UnexpectedSymbol   => println!( "Unexpected symbol was observed in the input file." ),
	}

	println!( "{}", "\n\x1b[31;1m!!! Program halted !!!\x1b[0m\n" );

	process::exit( 1 );
}
