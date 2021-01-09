macro_rules! input {
	(source = $s:expr, $($r:tt)*) => {
		let mut iter = $s.split_whitespace();
		let mut next = || { iter.next().unwrap() };
		input_inner!{next, $($r)*}
	};
	($($r:tt)*) => {
		let stdin = std::io::stdin();
		let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
		let mut next = move || -> String{
			bytes
				.by_ref()
				.map(|r|r.unwrap() as char)
				.skip_while(|c|c.is_whitespace())
				.take_while(|c|!c.is_whitespace())
				.collect()
		};
		input_inner!{next, $($r)*}
	};
}

macro_rules! input_inner {
	($next:expr) => {};
	($next:expr, ) => {};

	($next:expr, $var:ident : $t:tt $($r:tt)*) => {
		let $var = read_value!($next, $t);
		input_inner!{$next $($r)*}
	};
}

macro_rules! read_value {
	($next:expr, ( $($t:tt),* )) => {
		( $(read_value!($next, $t)),* )
	};

	($next:expr, [ $t:tt ; $len:expr ]) => {
		(0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
	};

	($next:expr, chars) => {
		read_value!($next, String).chars().collect::<Vec<char>>()
	};

	($next:expr, usize1) => {
		read_value!($next, usize) - 1
	};

	($next:expr, $t:ty) => {
		$next().parse::<$t>().expect("Parse error")
	};
}

fn main() {
	input!{
		n: u32,
		v: [u32; n],
	}
	let mut v = v;
	v.sort_by(|x, y| x.cmp(y).reverse());
	let mut ans = 0;
	// JavaScriptと違って↓の書き方はNG
	// 実行時にindexに対応する要素があるかを判定するのはRustの思想から反する
	// while v[idx] {
	// 	ans += v[idx] - (if v[idx + 1] { v[idx + 1] } else { 0 });
	// 	idx += 2;
	// }
	for (i, &x) in v.iter().enumerate() {
		if i % 2 == 0 {
			ans += x;
		} else {
			ans -= x;
		}
	}
	println!("{}", ans);
}