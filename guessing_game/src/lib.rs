use std::{
    cmp::Ordering,
    io::{BufRead, Write},
};

pub fn do_guess_random<R, W>(mut reader: R, mut writer: W, num: i32) -> Result<(), std::io::Error>
where
    R: BufRead,
    W: Write,
{
    writeln!(&mut writer, "The random number is: {num}")?;
    writeln!(&mut writer, "Enter a number from 1-100: ")?;
    loop {
        let mut guess = String::new();
        reader.read_line(&mut guess).expect("failed to read stdin");
        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                writeln!(&mut writer, "You must enter a number.")?;
                continue;
            }
        };

        match num.cmp(&guess) {
            Ordering::Equal => {
                writeln!(&mut writer, "You guessed correctly!")?;
                return Ok(());
            }
            Ordering::Greater => writeln!(&mut writer, "The number is greater")?,
            Ordering::Less => writeln!(&mut writer, "The number is lower")?,
        }
    }
}

#[test]
fn test_in_memory() {
    let guess = b"10";
    let mut output = Vec::new();

    let res = do_guess_random(&guess[..], &mut output, 10);

    assert!(res.is_ok());
    insta::assert_debug_snapshot!(String::from_utf8(output));
}

#[test]
fn test_higher_lower() {
    let guess = b"9\n11\n10";
    let mut output = Vec::new();

    let res = do_guess_random(&guess[..], &mut output, 10);

    assert!(res.is_ok());
    insta::assert_debug_snapshot!(String::from_utf8(output));
}

#[test]
fn test_bad_input() {
    let guess = b"not a number\n10";
    let mut output = Vec::new();

    let res = do_guess_random(&guess[..], &mut output, 10);

    assert!(res.is_ok());
    insta::assert_debug_snapshot!(String::from_utf8(output));
}
