
pub struct GapPenalty {
	pub gap_pen_list : Vec<f64>,
}

impl GapPenalty {
	pub fn new() -> GapPenalty {

		let gap_pen_list : Vec<f64> = Vec::new();

		GapPenalty {
			gap_pen_list : gap_pen_list,
		}
	}

	pub fn calc_gap_penalty(
		&mut self,
		site_list   : &Vec<String>,
		weight_list : &Vec<f64>,
	) {

		let num_site : usize = ( *site_list ).len();
		self.gap_pen_list    = vec![ 0.0; num_site ];

		/*
		 * Calculate simple gap penalties taking accont of sequence weighting.
		 * num_site = Number of the site
		 * site     = A site
		 * gap_pen_list = A list of gap penalties
		*/
		for i in 0 .. num_site {
			let site : &String = &( ( *site_list )[ i ] );
			( self.gap_pen_list )[ i ] = gap_penalty( site, weight_list );
		}

		( self.gap_pen_list ).shrink_to_fit();
	}
}

fn gap_penalty( site : &String, weight_list : &Vec<f64> ) -> f64 {

	let aa_list : Vec<char> = ( *site ).chars().collect();
	let mut gap_sum : f64 = 0.0;

	for i in 0 .. aa_list.len() {
		if aa_list[ i ] == '-' {
			gap_sum += ( *weight_list )[ i ];
		}
	}

	/*
	 * Normalize the gap penalty.
	 * The more the gaps, the smaller the penalty value, calculates as follows;
	 * 1.0 - { (sum of weighting scores assinged to gap symbol) / (sum of weighting scores) }.
	 * The denominator (sum of weighting scores) MUST be 1.000.
	*/
	1.0 - ( gap_sum / 1.000 )
}
