/// Determines whether the supplied string is a valid ISBN number
// How to handle given list:
// Check the length of the list, should be 10 or 13
// If 13: endswith X if contains char aside from X False
// Convert String to binary
// Remove "-"
pub fn is_valid_isbn(isbn: &str) -> bool {
    let multiplier:[u32; 10] = [10,9,8,7,6,5,4,3,2,1];
    // Remove hyphens
    let isbn = isbn.replace("-","");

    // Check if the length of the ISBN is either 10 or 13
    if isbn.len() != 10 && isbn.len() != 13 {
        return false;
    }

    let mut total_sum = 0;

    for (i, c) in isbn.chars().enumerate() {
        // enumerate returns a tuple which first element is the index  and the second is the value
        // itererate over the list whether the 'X' is at the very end and contains only a number
       
        let x = if !c.is_digit(10) && !(c == 'X' && i == 9) {
            return false; 
        } else if c == 'X' && i == 9 {
            10
            // If it is X and it is at the last position x = 10
        } else {
            c.to_digit(10).unwrap_or(0)
            //Convert character to digits
            //to_digits return Option<u32> then unwrap_or will return the value Some(value) or None /0.
        };

        // Accumulate the total sum
        total_sum += x * multiplier[i];
    }

    // Check if the total sum is divisible by 11
    total_sum % 11 == 0
        
}  

