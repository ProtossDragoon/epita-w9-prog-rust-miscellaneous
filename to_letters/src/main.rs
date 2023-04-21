use structopt::StructOpt;

// You are free to define some constants.
const TERMS_0_TO_9: [&str; 10] = [
	"zero",
	"one",
	"two",
	"three",
	"four",
	"five",
	"six",
	"seven",
	"eight",
	"nine",
];
const TERMS_MUL_10: [&str; 10] = [
	"placeholder", // useless
	"ten",
	"twenty",
	"thirty",
	"forty",
	"fifty",
	"sixty",
	"seventy",
	"eighty",
	"ninety",
];
const TERMS_10_TO_19: [&str; 10] = [
	"ten", // useless
	"eleven",
	"twelve",
	"thirteen",
	"fourteen",
	"fifteen",
	"sixteen",
	"seventeen",
	"eighteen",
	"nineteen",	
];
const TERMS_MUL_1000: [&str; 7] = [
	"placeholder", // useless
	"thousand", 
	"million", 
	"billion", 
	"trillion", 
	"quadrillion",
	"quintillion"
];

#[derive(Debug, StructOpt)]
#[structopt(name = "Numbers to Words Converter",
			about = "Convert an integer into letters.")]
struct Arg
{
	/// A number which you want to read.
	number: u64,
}

fn main()
{
    let arg = Arg::from_args();
    println!("{}", to_letters(arg.number));
}

fn to_letters(n: u64) -> String
{
	if n == 0 {
		return String::from("zero")
	}
	let mut i = 0;
	let mut left = n;
	let mut letters: Vec<String> = vec![];
	while left > 0 {
		let mut term: Vec<String> = to_vectors_less_than_thousand((left % 1000) as u16); 
		if i != 0 {
			term.push(TERMS_MUL_1000[i as usize].to_owned());
		}
		if !term.is_empty() {
			letters.push(term.join(" "));
		}
		left /= 1000;
		i += 1;
	}
	letters.iter().rev().map(|x| x.to_owned()).collect::<Vec<String>>().join(" ")
}

fn to_vectors_less_than_thousand(n: u16) -> Vec<String>
{   
    let n_tuple = (n / 100 as u16, (n % 100) / 10 as u16, n % 10 as u16);
    let mut hundred: Vec<String> = vec![];
    match n_tuple {
        (i, _, _) if i > 0 => hundred.push(String::from(TERMS_0_TO_9[i as usize])),
        _ => {}
    }; 
    if !hundred.is_empty() {
        hundred.push(String::from("hundred"));
    }
    let mut leftover: Vec<String> = vec![];
    match n_tuple {
        (_, 0, j) if j != 0 => leftover.push(String::from(TERMS_0_TO_9[j as usize])),
        (_, 1, j) if j != 0 => leftover.push(String::from(TERMS_10_TO_19[j as usize])),
        (_, i, 0) if i != 0 => leftover.push(String::from(TERMS_MUL_10[i as usize])),
        (_, i, j) if j != 0 => leftover.push(format!("{}-{}", TERMS_MUL_10[i as usize], TERMS_0_TO_9[j as usize])),
        _ => {}
    };
    if !hundred.is_empty() && !leftover.is_empty() {
        hundred.push(String::from("and"));
    }
    hundred.iter().chain(leftover.iter()).map(|x| x.to_owned()).collect::<Vec<String>>()
}

#[allow(unused)]
fn to_letters_less_than_thousand(n: u16) -> String
{
	if n == 0 {
		return String::from(TERMS_0_TO_9[n as usize])
	}
	if n >= 1000 {
		panic!();
	}
	to_vectors_less_than_thousand(n).join(" ")
}

#[cfg(test)]
mod tests
{
    use super::*;
	
	#[test]
	fn test_to_letters_less_than_thousand_under_10()
	{
        assert_eq!(to_letters_less_than_thousand(1), "one");
        assert_eq!(to_letters_less_than_thousand(2), "two");
        assert_eq!(to_letters_less_than_thousand(3), "three");
        assert_eq!(to_letters_less_than_thousand(4), "four");
        assert_eq!(to_letters_less_than_thousand(5), "five");
        assert_eq!(to_letters_less_than_thousand(6), "six");
        assert_eq!(to_letters_less_than_thousand(7), "seven");
        assert_eq!(to_letters_less_than_thousand(8), "eight");
        assert_eq!(to_letters_less_than_thousand(9), "nine");
	}

	#[test]
	fn test_to_letters_less_than_thousand_over_10()
	{
		assert_eq!(to_letters_less_than_thousand(10), "ten");
        assert_eq!(to_letters_less_than_thousand(20), "twenty");
        assert_eq!(to_letters_less_than_thousand(21), "twenty-one");
        assert_eq!(to_letters_less_than_thousand(30), "thirty");
        assert_eq!(to_letters_less_than_thousand(32), "thirty-two");
        assert_eq!(to_letters_less_than_thousand(40), "forty");
        assert_eq!(to_letters_less_than_thousand(43), "forty-three");
        assert_eq!(to_letters_less_than_thousand(50), "fifty");
        assert_eq!(to_letters_less_than_thousand(54), "fifty-four");
        assert_eq!(to_letters_less_than_thousand(60), "sixty");
        assert_eq!(to_letters_less_than_thousand(65), "sixty-five");
        assert_eq!(to_letters_less_than_thousand(70), "seventy");
        assert_eq!(to_letters_less_than_thousand(76), "seventy-six");
        assert_eq!(to_letters_less_than_thousand(80), "eighty");
        assert_eq!(to_letters_less_than_thousand(87), "eighty-seven");
        assert_eq!(to_letters_less_than_thousand(90), "ninety");
        assert_eq!(to_letters_less_than_thousand(98), "ninety-eight");
	}

	#[test]
	fn test_to_letters_less_than_thousand_teen()
	{
        assert_eq!(to_letters_less_than_thousand(11), "eleven");
        assert_eq!(to_letters_less_than_thousand(12), "twelve");
        assert_eq!(to_letters_less_than_thousand(13), "thirteen");
        assert_eq!(to_letters_less_than_thousand(14), "fourteen");
        assert_eq!(to_letters_less_than_thousand(15), "fifteen");
        assert_eq!(to_letters_less_than_thousand(16), "sixteen");
        assert_eq!(to_letters_less_than_thousand(17), "seventeen");
        assert_eq!(to_letters_less_than_thousand(18), "eighteen");
        assert_eq!(to_letters_less_than_thousand(19), "nineteen");
	}

	#[test]
	fn test_to_letters_less_than_thousand_over_100()
	{
        assert_eq!(to_letters_less_than_thousand(100), "one hundred");
        assert_eq!(to_letters_less_than_thousand(101), "one hundred and one");
        assert_eq!(to_letters_less_than_thousand(115), "one hundred and fifteen");
        assert_eq!(to_letters_less_than_thousand(165), "one hundred and sixty-five");
        assert_eq!(to_letters_less_than_thousand(200), "two hundred");
        assert_eq!(to_letters_less_than_thousand(277), "two hundred and seventy-seven");
        assert_eq!(to_letters_less_than_thousand(580), "five hundred and eighty");
        assert_eq!(to_letters_less_than_thousand(999), "nine hundred and ninety-nine");
	}

    #[test]
    fn test_quintillions()
    {
        assert_eq!(to_letters(0), "zero");
        assert_eq!(to_letters(1), "one");
        assert_eq!(to_letters(2), "two");
        assert_eq!(to_letters(3), "three");
        assert_eq!(to_letters(4), "four");
        assert_eq!(to_letters(5), "five");
        assert_eq!(to_letters(6), "six");
        assert_eq!(to_letters(7), "seven");
        assert_eq!(to_letters(8), "eight");
        assert_eq!(to_letters(9), "nine");

        assert_eq!(to_letters(10), "ten");
        assert_eq!(to_letters(11), "eleven");
        assert_eq!(to_letters(12), "twelve");
        assert_eq!(to_letters(13), "thirteen");
        assert_eq!(to_letters(14), "fourteen");
        assert_eq!(to_letters(15), "fifteen");
        assert_eq!(to_letters(16), "sixteen");
        assert_eq!(to_letters(17), "seventeen");
        assert_eq!(to_letters(18), "eighteen");
        assert_eq!(to_letters(19), "nineteen");
        assert_eq!(to_letters(20), "twenty");
        assert_eq!(to_letters(21), "twenty-one");
        assert_eq!(to_letters(30), "thirty");
        assert_eq!(to_letters(32), "thirty-two");
        assert_eq!(to_letters(40), "forty");
        assert_eq!(to_letters(43), "forty-three");
        assert_eq!(to_letters(50), "fifty");
        assert_eq!(to_letters(54), "fifty-four");
        assert_eq!(to_letters(60), "sixty");
        assert_eq!(to_letters(65), "sixty-five");
        assert_eq!(to_letters(70), "seventy");
        assert_eq!(to_letters(76), "seventy-six");
        assert_eq!(to_letters(80), "eighty");
        assert_eq!(to_letters(87), "eighty-seven");
        assert_eq!(to_letters(90), "ninety");
        assert_eq!(to_letters(98), "ninety-eight");

        assert_eq!(to_letters(100), "one hundred");
        assert_eq!(to_letters(101), "one hundred and one");
        assert_eq!(to_letters(115), "one hundred and fifteen");
        assert_eq!(to_letters(165), "one hundred and sixty-five");
        assert_eq!(to_letters(200), "two hundred");
        assert_eq!(to_letters(277), "two hundred and seventy-seven");
        assert_eq!(to_letters(580), "five hundred and eighty");
        assert_eq!(to_letters(999), "nine hundred and ninety-nine");

        assert_eq!(to_letters(1_000), "one thousand");
        assert_eq!(to_letters(5_454), "five thousand four hundred and fifty-four");
        assert_eq!(to_letters(9_999), "nine thousand nine hundred and ninety-nine");

        assert_eq!(to_letters(100_002), "one hundred thousand two");
        assert_eq!(to_letters(200_100_003), "two hundred million one hundred thousand three");

        assert_eq!(to_letters(18_446_744_073_709_551_615),
            "eighteen quintillion four hundred and forty-six \
             quadrillion seven hundred and forty-four trillion \
             seventy-three billion \
             seven hundred and nine million \
             five hundred and fifty-one thousand \
             six hundred and fifteen");
    }
}

