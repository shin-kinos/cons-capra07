
use std::env;
use std::process;

pub struct Options {
	pub input    : String,
	pub output   : String,
	pub weight   : String,
	pub tolerate : String,
	pub bgdist   : String,
	pub colorize : String,
}

impl Options {
	pub fn new() -> Options
	{
		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_o : &String = &String::new();
		let mut arg_w : &String = &String::from( "hen"      );
		let mut arg_t : &String = &String::from( "yes"      );
		let mut arg_b : &String = &String::from( "blosum62" );
		let mut arg_c : &String = &String::from( "no"       );

		if argc < 5 { show_usage( &argv[ 0 ] ) };

		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"-i" => { i += 1; arg_i = &argv[ i ]; }
				"-o" => { i += 1; arg_o = &argv[ i ]; }
				"-w" => { i += 1; arg_w = &argv[ i ]; }
				"-t" => { i += 1; arg_t = &argv[ i ]; }
				"-b" => { i += 1; arg_b = &argv[ i ]; }
				"-c" => { i += 1; arg_c = &argv[ i ]; }
				"-h" => { show_usage( &argv[ 0 ] );   }
				_    => { show_usage( &argv[ 0 ] );   }
			}
			i += 1;
		}

		match ( *arg_w ).as_str() {
			"hen" | "va" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *arg_t ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		match ( *arg_b ).as_str() {
			"blosum62"  => (),
			"swissprot" => (),
			"extra"     => (),
			"membrane"  => (),
			"intra"     => (),
			"jtt"       => (),
			"wag"       => (),
			"lg"        => (),
			"equal"     => (),
			_           => show_usage( &argv[ 0 ] ),
		}

		match ( *arg_c ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		Options {
			input    : arg_i.to_string(),
			output   : arg_o.to_string(),
			weight   : arg_w.to_string(),
			tolerate : arg_t.to_string(),
			bgdist   : arg_b.to_string(),
			colorize : arg_c.to_string(),
		}
	}

	pub fn show_parameter( &self )
	{
		println!( "\nParameter set :" );
		println!( "===========================================" );
		println!( "Input filename    : {}", self.input          );
		println!( "Onput filename    : {}", self.output         );
		println!( "Weighting method  : {}", self.weight         );
		println!( "Non-standard AA   : {}", self.tolerate       );
		println!( "B.G. distribution : {}", self.bgdist         );
		println!( "Colorize AA       : {}", self.colorize       );
		println!( "===========================================" );
	}
}

fn show_usage( arg : &String )
{
	println!( "Usage: {} [Options] \n\nOptions :\n\n", *arg );
	println!( "    -i    Input filename in aligned Multi-FASTA format, REQUIRED." );
	println!( "    -o    Onput filename, REQUIRED." );
	println!( "    -w    Method of sequence weighting ('hen' or 'va', default 'hen').
              hen : Position-Based method by Henikoff and Henikoff
              va  : Distance-Based method by Vingron and Argos" );
	println!( "    -t    Tolerate non-standard AA types (such as B, Z and X) in input file ('yes' or 'no', default 'yes').
              yes : All non-standard AAs are converted to gaps.
              no  : The program halts if the input file includes non-standard AA types." ); 
	println!( "    -b    Back ground distribution in the relative entropy (default 'blosum62').
              blosum62  : BLOSUM62
              swissprot : Swiss-Prot
              extra     : AA composition in extracellular proteins
              membrane  : AA composition in membrane proteins
              intra     : AA composition in intracellular proteins
              jtt       : JTT
              wag       : WAG
              lg        : LG
              equal     : No background distribution, equal rate (= 0.05)" );
	println!( "    -c    Colorize each AA displayed on the terminal based on their stereochemical properties ('yes' or 'no', default 'no')."  );
	println!( "    -h    Print this help, ignore all other arguments." );
	println!( "\n" );

	process::exit( 1 );
}
