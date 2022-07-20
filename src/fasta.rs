
use std::fs::read_to_string;

use crate::error;
use crate::error::ErrorType;
use crate::options::Tolerate;

pub struct Fasta {
	pub title_list : Vec<String>,
	pub seq_list   : Vec<String>,
	pub site_list  : Vec<String>,
}

impl Fasta {
	pub fn new() -> Fasta {

		let title_list : Vec<String> = Vec::new();
		let seq_list   : Vec<String> = Vec::new();
		let site_list  : Vec<String> = Vec::new();

		Fasta {
			title_list : title_list,
			seq_list   : seq_list,
			site_list  : site_list,
		}
	}

	pub fn read_fasta_info( &mut self, arg_i : &String ) {

		let fin = read_to_string( ( *arg_i ).as_str() ).expect( "FAILED to open input file" );

		/* Temporary String to conbine a sequence line separated by "\n" */
		let mut segment : Vec<String> = Vec::new();

		for line in fin.lines() {
			if line.starts_with( ">" ) && segment.is_empty() {
				( self.title_list ).push( line.to_string() );
			} else if line.starts_with( ">" ) && !segment.is_empty() {
				( self.title_list ).push( line.to_string() );
				( self.seq_list ).push( segment.concat() );
				segment.clear();
			} else {
				segment.push( line.to_string() );
			}
		}
		( self.seq_list ).push( segment.concat() );
		//segment.clear();
		//segment.shrink_to_fit();
		
		( self.title_list ).shrink_to_fit();
		( self.seq_list   ).shrink_to_fit();
		( self.site_list  ).shrink_to_fit();
	}

	pub fn check_fasta_info( &mut self, tolerate : &Tolerate ) {

		let num_title : usize = ( self.title_list ).len();
		let num_seq   : usize = ( self.seq_list   ).len();

		/* Tolerate non-std residues or not. */
		for i in 0 .. num_seq {
			let sequence : &String = &( self.seq_list[ i ] );
			match *tolerate {
				Tolerate::Yes => self.seq_list[ i ] = convert_to_gap( sequence, i + 1 ),
				Tolerate::No  => check_symbol( sequence, i + 1 ),
			}
		}

		/* Check the Multi FASTA format. */
		if num_seq != num_title { error::error_bomb( ErrorType::SeqTitleNotSame ); }

		/* Check the sequences are aligned. */
		for i in 1 .. num_seq {
			if ( self.seq_list[ 0 ] ).len() != ( self.seq_list[ i ] ).len() {
				error::error_bomb( ErrorType::SeqLenNotSame );
			}
		}

	}

	pub fn get_site_list( &mut self ) {

		let num_seq  : usize = ( self.seq_list ).len();
		let num_site : usize = ( self.seq_list[ 0 ] ).to_string().len();

		println!( "Number of the sequences : {}", num_seq  );
		println!( "Number of the sites     : {}", num_site );

		let mut site : Vec<String> = Vec::new();

		for i in 0 .. num_site {
			for j in 0 .. num_seq {
				let segment : Vec<char> = ( self.seq_list[ j ] ).chars().collect();
				site.push( segment[ i ].to_string() );
			}
			( self.site_list ).push( site.concat() );
			site.clear();
		}
		//site.shrink_to_fit();
	}

}

fn convert_to_gap( sequence : &String, seq_order : usize ) -> String {

	let mut aa_list : Vec<char> = ( *sequence ).chars().collect();

	for i in 0 .. aa_list.len() {
		let aa : char = aa_list[ i ];
		match aa {
			'A'|'R'|'N'|'D'|'C'|'Q'|'E'|'G'|'H'|'I'|'L'|'K'|'M'|'F'|'P'|'S'|'T'|'W'|'Y'|'V'|'-' => (),
			'B'|'Z'|'X'|'U'|'O' => {
				println!( "\nNOTE :");
				println!( "Non-standard residue was observed in sequence {} : '{}'", seq_order, aa );
				println!( "'{}' was converted into gap.", aa );
				println!( "" );
				aa_list[ i ] = '-';
			},
			_ => {
				println!( "\nNOTE :" );
				println!( "Unexpected symbol was observed in sequence {} : '{}'", seq_order, aa );
				println!( "'{}' was converted into gap.", aa );
				println!( "" );
				aa_list[ i ] = '-';
			},
		}
	}

	/* Convert Vec<char> into String. */
	aa_list.iter().collect()
}

fn check_symbol( sequence : &String, seq_order : usize ) {

	let aa_list : Vec<char> = ( *sequence ).chars().collect();

	for i in 0 .. aa_list.len() {
		let aa : char = aa_list[ i ];
		match aa {
			'A'|'R'|'N'|'D'|'C'|'Q'|'E'|'G'|'H'|'I'|'L'|'K'|'M'|'F'|'P'|'S'|'T'|'W'|'Y'|'V'|'-' => (),
			'B'|'Z'|'X'|'U'|'O' => {
				println!( "\nFATAL :" );
				println!( "Non-standard residue was observed in sequence {} : '{}'", seq_order, aa );
				println!( "" );
				error::error_bomb( ErrorType::NonStandardResidue );
			},
			_ => {
				println!( "\nFATAL :" );
				println!( "Unexpected symbol was observed in sequence {} : '{}'", seq_order, aa );
				println!( "" );
				error::error_bomb( ErrorType::UnexpectedSymbol );
			},
		}
	}

}
