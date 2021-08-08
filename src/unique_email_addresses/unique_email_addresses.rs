// Unique Email Addresses
// URL: https://leetcode.com/explore/featured/card/google/67/sql-2/3044/
// 
// Every valid email consists of a local name and a domain name, separated by the '@' sign. 
// Besides lowercase letters, the email may contain one or more '.' or '+'.

// For example, in "alice@leetcode.com", "alice" is the local name, and "leetcode.com" is the domain name.
// If you add periods '.' between some characters in the local name part of an email address,
// mail sent there will be forwarded to the same address without dots in the local name. Note that this rule does not apply to domain names.

// For example, "alice.z@leetcode.com" and "alicez@leetcode.com" forward to the same email address.
// If you add a plus '+' in the local name, everything after the first plus sign will be ignored. 
// This allows certain emails to be filtered. Note that this rule does not apply to domain names.

// For example, "m.y+name@email.com" will be forwarded to "my@email.com".
// It is possible to use both of these rules at the same time.

// Given an array of strings emails where we send one email to each email[i], return the number of different addresses that actually receive mails.

use std::collections::HashSet;

struct Solution {

}
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut email_set: HashSet<String> = HashSet::new();

        fn strip_extra_chars (email: String) -> String {
            let email = email.split("+").next().unwrap();
            let email = email.replace(".", "");
            return email;
        }
        for e in emails {
            let mut split = e.split("@");
            let local: String = split.next().unwrap().to_string();
            let domain: String = split.next().unwrap().to_string();

            let local = strip_extra_chars(local);

            email_set.insert(format!("{}@{}", local, domain));
        }

        return email_set.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input: Vec<String> = vec!(
        "test.email+alex@leetcode.com".to_string(), 
        "test.e.mail+bob.cathy@leetcode.com".to_string(), 
        "testemail+david@lee.tcode.com".to_string());
        let result = Solution::num_unique_emails(input);

        assert_eq!(result, 2);
    }
}