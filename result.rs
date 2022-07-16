
use std::fs::File;
use std::io::Write;
//use colored::*;
use crate::error;
use crate::error::ErrorType;

static YELLOW  : &str = "\x1b[103;30m";
static CYAN    : &str = "\x1b[106;30m";
static GREEN   : &str = "\x1b[102;30m";
static BLUE    : &str = "\x1b[104;37m";
static RED     : &str = "\x1b[101;37m";
static MAGENTA : &str = "\x1b[105;37m";
static RESET   : &str = "\x1b[0m";

pub fn show_result(
	site_list         : &Vec<String>,
	cons_capra07_list : &Vec<f64>,
	arg_c             : &String
) {

	if ( *site_list ).len() != ( *cons_capra07_list ).len() { error::error_bomb( ErrorType::SiteEntLenNotSame ); }

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
		println!( "{}",  YELLOW.to_string()  + "Aliphatic (A, V, L, I, M, C)" + RESET );
		println!( "{}",  CYAN.to_string()    + "Aromatic        (F, W, Y, H)" + RESET );
		println!( "{}",  GREEN.to_string()   + "Polar           (S, T, N, Q)" + RESET );
		println!( "{}",  BLUE.to_string()    + "Positive              (K, R)" + RESET );
		println!( "{}",  RED.to_string()     + "Negative              (D, E)" + RESET );
		println!( "{}",  MAGENTA.to_string() + "Special conformations (G, P)" + RESET );
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
			'A' | 'V' | 'L' | 'I' | 'M' | 'C' => print!( "{}", YELLOW.to_string()     + &( *symbol ).to_string() + RESET ), // .on_yellow().black() 
			'F' | 'W' | 'Y' | 'H'             => print!( "{}", CYAN.to_string()       + &( *symbol ).to_string() + RESET ), // .on_cyan().black()   
			'S' | 'T' | 'N' | 'Q'             => print!( "{}", GREEN.to_string()      + &( *symbol ).to_string() + RESET ), // .on_green().black()  
			'K' | 'R'                         => print!( "{}", BLUE.to_string()       + &( *symbol ).to_string() + RESET ), // .on_blue().black()   
			'D' | 'E'                         => print!( "{}", RED.to_string()        + &( *symbol ).to_string() + RESET ), // .on_red().black()    
			'G' | 'P'                         => print!( "{}", MAGENTA.to_string()    + &( *symbol ).to_string() + RESET ), // .on_magenta().black()
			'B' | 'Z' | 'J' | 'O'             => print!( "{}", "\x1b[93m".to_string() + &( *symbol ).to_string() + RESET ), // .yellow()            
			'X'                               => print!( "{}", "\x1b[91m".to_string() + &( *symbol ).to_string() + RESET ), // .red()               
			_                                 => print!( "{}",                           ( *symbol ).to_string()         ),
		}
	}
}
