/// The rank of an Ace.
pub const ACE: Rank = 1;
/// The rank of a Jack.
pub const JACK: Rank = 11;
/// The rank of a Queen.
pub const QUEEN: Rank = 12;
/// The rank of a King.
pub const KING: Rank = 13;

/// Indicates the rank of a card.
///
/// The ranks are as follows:
/// - An Ace has rank 1
/// - Numbered cards have the rank corresponding to their numbers
/// - A Jack has rank 11
/// - A Queen has rank 12
/// - A King has rank 13
///
/// Rank 0 and ranks greater than 13 are not valid card ranks.
///
/// Can be parsed with [`parse_rank`](fn.parse_rank.html).
pub type Rank = u8;

/// Converts a string to a [`Rank`](type.Rank.html).
///
/// The string should be either the rank as number between 1 and 13 or one of these strings that
/// represent the named cards:
/// "Ace", "A", "Jack", "J", "Queen", "Q", "King", "K" or "T", which represents 10.
/// The strings are case-insensitive.
///
/// # Examples
///
/// ```
/// use freecell::{parse_rank, ACE, JACK, KING};
///
/// assert_eq!(Ok(ACE), parse_rank("ace"));
/// assert_eq!(Ok(JACK), parse_rank("J"));
/// assert_eq!(Ok(10), parse_rank("t"));
/// assert_eq!(Ok(8), parse_rank("8"));
/// assert_eq!(Ok(KING), parse_rank("13"));
/// assert!(parse_rank("0").is_err());
/// assert!(parse_rank("14").is_err());
/// ```
pub fn parse_rank<S: Into<String>>(string: S) -> Result<Rank, String> {
    let string = string.into();
    match string.to_lowercase().trim() {
        "ace" => Ok(ACE),
        "a" => Ok(ACE),
        "t" => Ok(10),
        "jack" => Ok(JACK),
        "j" => Ok(JACK),
        "queen" => Ok(QUEEN),
        "q" => Ok(QUEEN),
        "king" => Ok(KING),
        "k" => Ok(KING),

        // rank is not a word => try to parse it as u8
        _ => match string.trim().parse::<u8>() {
            Err(_) => Err(format!("Rank is neither named rank nor integer: \"{}\"", string)),

            // successfully parsed number => check whether it is in the correct range
            Ok(rank) => match rank {
                0 => Err("Rank cannot be 0".to_string()),
                1..=13 => Ok(rank),
                _ => Err(format!("Rank cannot be greater than 13 (rank is {})", rank)),
            },
        },
    }
}
