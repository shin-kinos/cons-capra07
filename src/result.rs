
use std::fs::File;
use std::io::Write;
use colored::*;
use crate::error;

pub fn show_result(
	site_list         : &Vec<String>,
	cons_capra07_list : &Vec<f64>,
	arg_c             : &String
) {

	if ( *site_list ).len() != ( *cons_capra07_list ).len() { error::error_bomb( "site_ent_len_not_same" ); }

	println!( "\nResult :\n" );

	if ( *arg_c ).as_str() == "yes" {
		println!( "Colorize :" );
		/*
		println!( "{}", "Aliphatic (A, V, L, I, M, C)".on_truecolor(221, 255,   0).truecolor(0,   0,   0) );
		println!( "{}",        "Aromatic (F, W, Y, H)".on_truecolor(  0, 255, 243).truecolor(0,   0,   0) );
		println!( "{}",           "Polar (S, T, N, Q)".on_truecolor(  0, 255, 136).truecolor(0,   0,   0) );
		println!( "{}",              "Positive (K, R)".on_truecolor(  0, 149, 255).truecolor(0,   0,   0) );
		println!( "{}",              "Negative (D, E)".on_truecolor(255,  37,  37).truecolor(0,   0,   0) );
		println!( "{}", "Special conformations (G, P)".on_truecolor(255,   0, 255).truecolor(0,   0,   0) );
		*/
		println!( "{}", "Aliphatic (A, V, L, I, M, C)".on_yellow().black()  );
		println!( "{}",        "Aromatic (F, W, Y, H)".on_cyan().black()    );
		println!( "{}",           "Polar (S, T, N, Q)".on_green().black()   );
		println!( "{}",              "Positive (K, R)".on_blue().black()    );
		println!( "{}",              "Negative (D, E)".on_red().black()     );
		println!( "{}", "Special conformations (G, P)".on_magenta().black() );
		println!( "" );

		println!( "num\tcons\tsite" );
		for i in 0 .. ( *cons_capra07_list ).len() {
			print!( "{}\t", i + 1 );
			print!( "{:.3}\t", ( *cons_capra07_list )[ i ] );
			colorize( &( ( *site_list )[ i ] ) );
			println!( "" );
		}


	} else if ( *arg_c ).as_str() == "no" {
		/* Not colorize. */
		println!( "num\tcons\tsite" );
		for i in 0 .. ( *cons_capra07_list ).len() {
			println!( "{}\t{:.3}\t{}", i + 1, ( *cons_capra07_list )[ i ], ( *site_list )[ i ] );
		}
	}
}

pub fn save_result(
	site_list         : &Vec<String>,
	cons_capra07_list : &Vec<f64>,
	arg_o             : &String
) {

	let mut fout = File::create( ( *arg_o ).as_str() ).expect( "FAILED to open output file" );

	writeln!( fout, "{}", "num\tcons\tsite" ).expect( "FAILED to write" );

	for i in 0 .. ( *cons_capra07_list ).len() {
		writeln!( fout, "{}\t{:.3}\t{}", i + 1, ( *cons_capra07_list )[ i ], ( *site_list )[ i ] ).expect( "FAILED to write" );
	}

	println!( "\nThe output file was correctly written.\n" );
}

fn colorize( arg : &String ) {

	let sequence : Vec<char> = ( *arg ).chars().collect();
	//println!("{:?}", sequence);

	for symbol in sequence.iter() {
		match *symbol {
			'A' | 'V' | 'L' | 'I' | 'M' | 'C' => print!( "{}", ( *symbol ).to_string().on_yellow().black()  ),
			'F' | 'W' | 'Y' | 'H'             => print!( "{}", ( *symbol ).to_string().on_cyan().black()    ),
			'S' | 'T' | 'N' | 'Q'             => print!( "{}", ( *symbol ).to_string().on_green().black()   ),
			'K' | 'R'                         => print!( "{}", ( *symbol ).to_string().on_blue().black()    ),
			'D' | 'E'                         => print!( "{}", ( *symbol ).to_string().on_red().black()     ),
			'G' | 'P'                         => print!( "{}", ( *symbol ).to_string().on_magenta().black() ),
			'B' | 'Z' | 'J' | 'O'             => print!( "{}", ( *symbol ).to_string().yellow()             ),
			'X'                               => print!( "{}", ( *symbol ).to_string().red()                ),
			_                                 => print!( "{}", ( *symbol ).to_string()                      ),
		}
	}
}
