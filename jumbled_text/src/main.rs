use std::env;

const SEPARATORS: &str = " ,;:!?./%*$=+)@_-('\"&1234567890\r\n";

fn main()
{
	let args: Vec<String> = env::args().collect();
	let s: &str;
	if args.len() != 2 {
		s = "This is the default text.";
	}
	else {
		s = &args[1];
	}
	println!("{}", s);
	println!("{}", mix(s));
}

fn mix(s: &str) -> String
{
	let mut v: Vec<char> = s.chars().collect::<Vec<char>>();
	v.split_mut(|x| SEPARATORS.contains(*x))
	 .filter(|x| x.len() > 1)
	 .for_each(|x| {
        let len = x.len();
        for e in x[1..len-1].chunks_exact_mut(2) {
            e.swap(0, 1)
        };
    });
	v.iter().collect::<String>()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_mix()
    {
		assert_eq!(mix("This is the default text."), "Tihs is the dfeualt txet.");
		assert_eq!(mix("Hello World!"), "Hlelo Wrold!");
		assert_eq!(mix("I am not a number! I'm a free man!"), "I am not a nmuebr! I'm a fere man!");
		assert_eq!(mix("I was born ready!"), "I was bron raedy!");
		assert_eq!(mix("hello"), "hlelo");
		assert_eq!(mix("world"), "wrold");
		assert_eq!(mix("worrld"), "wrolrd");
    }
}

