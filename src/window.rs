
use crate::error;
use crate::error::ErrorType;

pub fn check_window_size( window_size : usize, num_site : usize ) {
	if ( window_size * 3 ) > num_site { error::error_bomb( ErrorType::TooLargeWindowSize ); }
}

pub fn moving_average( cons_capra07_list : &Vec<f64>, window_size : usize ) -> Vec<f64> {

	let mut window_scores : Vec<f64> = ( *cons_capra07_list ).clone();

	for i in ( window_size + 0 ) .. ( ( *cons_capra07_list ).len() - window_size ) {
		let mut window_sum : f64 = 0.0;
		//print!( "Window of {} : ", ( *cons_capra07_list )[ i ] );
		for j in ( i - window_size ) .. ( i + window_size + 1 ) {
			if i != j {
				window_sum += ( *cons_capra07_list )[ j ];
				//print!( "{}, ", cons_capra07_list[ j ] );
			}
		}
		let window_mean : f64 = window_sum / ( window_size * 2 ) as f64;
		window_scores[ i ] = ( window_scores[ i ] * 0.5 ) + ( window_mean * 0.5 );
	}

	window_scores

}