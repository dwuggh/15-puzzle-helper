mod lib;

use std::time::Instant;
use clap::{Arg, App, Clap};

use crate::lib::{solver, scrambler};


#[derive(Clap)]
#[clap(version = "0.1.0", author = "dwuggh <dwuggh@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

#[derive(Clap)]
enum SubCmd {
    Scramble(Scramble),
}

#[derive(Clap)]
struct Scramble {
    
}

fn main() {
    // let b = scrambler::scramble_random(4);
    // let b_clone = b.clone();
    // let timer = Instant::now();
    // // let re: GameBoard = search_A_star(b);
    // let re = solver::rank_reduction_search(b);
    // // dbg!(re);
    // println!("{:?}", re);
    // println!("{}", b_clone.board);
    // println!("time: {}", timer.elapsed().as_secs_f64());

    let matches = App::new("fifteen-puzzle-helper")
	.version("0.1.0")
	.author("dwuggh <dwuggh@gmail.com>")
	.about("solver and scramble generator for 15-puzzle")
	.subcommand(
	    App::new("scramble")
		    .about("scramble the 15 puzzle")
		    .arg(Arg::new("rank")
			 .short('r')
			 .long("rank")
			 .about("rank of the puzzle")
			 .required(true)
			 .takes_value(true)
			 .default_value("4")
		    ).arg(Arg::new("print-style")
			  .long("print-style")
			  .about("printing style. Valid options are: pretty, compact, slidysim")
			  .default_value("pretty")
		    )
	).subcommand(
	    App::new("solve")
		.about("solve a scramble")
		.arg(Arg::new("rank")
			 .short('r')
			 .long("rank")
			 .about("rank of the puzzle")
			 .required(true)
			 .takes_value(true)
			 .default_value("4")
		).arg(Arg::new("input")
		      .about("the scramble input")
		      .required(true)
		      .default_value("random")
		      .index(1)
		)
	)
	.get_matches();

    if let Some(sub_scm) = matches.subcommand_matches("scramble") {
	let rank = sub_scm.value_of("rank").unwrap().parse::<u64>();
	match rank {
	    Ok(r) => {
		let scramble = scrambler::scramble_random(r);
		let style = sub_scm.value_of("print-style").unwrap();
		println!("{}", scramble);
	    }
	    Err(_) => {
		println!("rank must be a non-negative integer.");
		return
	    }
	}
    }

    if let Some(sub_scm) = matches.subcommand_matches("solve") {
	let rank = sub_scm.value_of("rank").unwrap().parse::<u64>();
	match rank {
	    Ok(r) => {
		match sub_scm.value_of("input").unwrap() {
		    "random" => {
			let scramble = scrambler::scramble_random(r);
			println!("{}", scramble);
			let ans = solver::solve(scramble, solver::SolveMethod::Rank_Reduction);
			println!("{:?}", ans);
		    }
		    _ => {}
		}
		let scramble = scrambler::scramble_random(r);
		println!("{}", scramble);
	    }
	    Err(_) => {
		println!("rank must be a non-negative integer.");
		return
	    }
	}
    }

    
}
