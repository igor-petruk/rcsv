extern mod extra;

use std::io::*;
use self::extra::sort::*;
use self::extra::getopts::*;
use std::os;

mod regex;

#[deriving(Eq, Clone)]
struct SortableLine<T>{
	comparable_part: T,
	line: ~str
}

impl <T:Ord> Ord for SortableLine<T>{
	fn lt(&self, other: &SortableLine<T>) -> bool{
		return self.comparable_part.lt(&other.comparable_part);
	}
}

fn main(){
	let compiled = regex::compile(&".?*\\(wo\\).*");
	
	match compiled {
		~regex::CompilationSuccess(compiled_regex) => {
			let matches = regex::rmatch(compiled_regex, "Hello wo world ", 5);
			for i in matches.iter(){
				println(fmt!("%?",i))
			}
		}
		~regex::CompilationError(er) => fail!(er)
	}


	//b let mut lines: ~[~SortableLine<~str>] = ~[];

 // 	let args = os::args();

 //    let opts = ~[
 //        optflag("g"),
 //        optflag("f"),
 //        optflag("r"),
 //        optflag("u")        
 //        // optflag("help")
 //    ];
 //    let matches = match getopts(args.tail(), opts) {
 //        Ok(m) => { m }
 //        Err(f) => { fail!(f.to_err_msg()) }
 //    };
    
 //    let numeric = matches.opt_present("g");
 //    let ignore_case = matches.opt_present("f");
 //    let reverse = matches.opt_present("r");
 //    let unique = matches.opt_present("u");

 //    println(fmt!("%? %? %? %?", numeric, ignore_case, reverse, unique));

	// while (!stdin().eof()){
	// 	let line_read = stdin().read_line();
	// 	let comparable_part = line_read.clone();
	// 	lines.push(~SortableLine{
	// 		line: line_read,
	// 		comparable_part: comparable_part
	// 	});
	// }

	// quick_sort3	(lines);

	// for line in lines.iter() {
	// 	println((*line).line)
	// }

	// let i: ~int = ~1;
	// let x: &int = i;
	// let y: int = *i;

	// println(fmt!("%? %?", x, i));

}