
pub const ACE: Rank = 1;
pub const JACK: Rank = 11;
pub const QUEEN: Rank = 12;
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
pub type Rank = u8;


/// Converts a String to a Rank.
///
/// The string should be either the rank as number between 1 and 13 or one of these strings that
/// represent the named cards:
/// "Ace", "A", "Jack", "J", "Queen", "Q", "King", "K" or "T", which represents 10.
/// The strings are case-insensitive.
///
/// # Examples
///
/// ```
/// use freecell::{rank_from_string, ACE, JACK, KING};
///
/// assert_eq!(Ok(ACE), rank_from_string(String::from("ace")));
/// assert_eq!(Ok(JACK), rank_from_string(String::from("J")));
/// assert_eq!(Ok(10), rank_from_string(String::from("t")));
/// assert_eq!(Ok(8), rank_from_string(String::from("8")));
/// assert_eq!(Ok(KING), rank_from_string(String::from("13")));
/// assert!(rank_from_string(String::from("0")).is_err());
/// assert!(rank_from_string(String::from("14")).is_err());
/// ```
pub fn rank_from_string(string: String) -> Result<Rank, String> {
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
            Err(_) => return Err(format!("Could not parse integer {}", string)),

            // successfully parsed number => check whether it is in the correct range
            Ok(rank) => match rank {
                0 => Err("Rank cannot be 0".to_string()),
                1...13 => Ok(rank),
                _ => Err(format!("Rank cannot be greater than 13 (rank is {})", rank)),
            }
        }
    }
}
