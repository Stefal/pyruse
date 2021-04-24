use super::Error;
use std::convert::TryFrom;
use std::string::ToString;

#[derive(Clone)]
pub struct EmailData {
  pub text: Option<String>,
  pub html: Option<String>,
  pub subject: String,
}
impl EmailData {
  pub fn new(subject: String) -> Self {
    EmailData {
      text: None,
      html: None,
      subject,
    }
  }
}

pub trait EmailPort {
  fn send(&mut self, email: EmailData) -> Result<(), Error>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct EmailAddress {
  as_string: String,
}

impl ToString for EmailAddress {
  fn to_string(&self) -> String {
    self.as_string.clone()
  }
}

impl Clone for EmailAddress {
  fn clone(&self) -> Self {
    EmailAddress {
      as_string: self.as_string.clone(),
    }
  }
  fn clone_from(&mut self, source: &Self) {
    self.as_string = source.as_string.clone();
  }
}

impl TryFrom<String> for EmailAddress {
  type Error = Error;
  fn try_from(as_string: String) -> Result<Self, Error> {
    if is_address_valid(&as_string) {
      Ok(EmailAddress { as_string })
    } else {
      Err(format!("Email {} is invalid", as_string).into())
    }
  }
}

impl Into<String> for EmailAddress {
  fn into(self) -> String {
    self.as_string
  }
}

enum EmlAddr {
  Name,
  LocalSpaceTest,
  LocalQuoteTest,
  Local,
  Arob,
  Host,
  End,
}

const MAX_LOCAL_SIZE: i32 = 64;
const MAX_DNS_PART_SIZE: i32 = 63;
const MAX_HOST_SIZE: i32 = 255;
const MAX_EMAIL_SIZE: usize = 254;

/*
https://tools.ietf.org/html/rfc3696 + Errata

Restrictions:
— No IP addresses (“xxx@[IP:ad:dre:ss]”)
— Partial non-ASCII characters support (Unicode)
 */
fn is_address_valid(addr: &str) -> bool {
  if addr.len() > MAX_EMAIL_SIZE {
    return false;
  }
  let mut all_num_top_dom = true;
  let mut dash_seen_last = false;
  let mut dot_seen_last = false;
  let mut quote_needed = false;
  let mut host_size = 0;
  let mut dns_part_size = 0;
  let mut local_size = 0;
  let mut with_brackets = false;
  let mut in_quotes = false;
  let mut state = EmlAddr::End;
  for c in addr.chars().rev() {
    match state {
      EmlAddr::Name => match c {
        ' ' => (),
        _ => return with_brackets,
      },
      EmlAddr::LocalSpaceTest => {
        if c == '\\' {
          local_size += 2;
          dot_seen_last = false;
          state = EmlAddr::Local;
        } else if with_brackets
          || dot_seen_last
          || local_size == 0
          || local_size > MAX_LOCAL_SIZE
          || c != ' '
        {
          return false;
        } else {
          state = EmlAddr::Name;
        }
      }
      EmlAddr::LocalQuoteTest => {
        if c == '\\' {
          local_size += 2;
          dot_seen_last = false;
          state = EmlAddr::Local;
        } else if !in_quotes || local_size == 0 || local_size > MAX_LOCAL_SIZE {
          return false;
        } else if with_brackets && c != '<' {
          return false;
        } else {
          state = EmlAddr::Name;
        }
      }
      EmlAddr::Local => {
        if quote_needed {
          if c != '\\' {
            return false;
          }
          quote_needed = false;
        } else {
          match c {
            '.' => {
              if !in_quotes && dot_seen_last {
                return false;
              }
            }
            '<' => {
              if !in_quotes {
                if !with_brackets || dot_seen_last || local_size == 0 || local_size > MAX_LOCAL_SIZE
                {
                  return false;
                }
                state = EmlAddr::Name;
              }
            }
            '\\' => {
              if local_size == 0 {
                quote_needed = true;
              }
            }
            '@' | ',' | '[' | ']' => {
              if !in_quotes {
                quote_needed = true;
              }
            }
            '"' => {
              state = EmlAddr::LocalQuoteTest;
              continue;
            }
            ' ' => {
              if !in_quotes {
                state = EmlAddr::LocalSpaceTest;
                continue;
              }
            }
            '!'..='~' => (),
            _ => {
              if !in_quotes {
                quote_needed = true;
              }
            }
          }
        };
        dot_seen_last = c == '.';
        local_size += 1;
      }
      EmlAddr::Arob => {
        match c {
          '"' => {
            in_quotes = true;
            local_size = -1;
          }
          '.' => return false,
          '@' | '\\' | ',' | '[' | ']' => quote_needed = true,
          '!'..='~' => (),
          _ => quote_needed = true,
        };
        local_size += 1;
        state = EmlAddr::Local;
      }
      EmlAddr::Host => {
        match c {
          '@' => {
            if dash_seen_last
              || all_num_top_dom
              || dns_part_size == 0
              || dns_part_size > MAX_DNS_PART_SIZE
              || host_size > MAX_HOST_SIZE
            {
              return false;
            }
            state = EmlAddr::Arob;
          }
          '.' => {
            if dash_seen_last
              || (host_size > 0 && (all_num_top_dom || dns_part_size == 0))
              || dns_part_size > MAX_DNS_PART_SIZE
            {
              return false;
            }
            dns_part_size = -1;
          }
          '0'..='9' => (),
          '-' => {
            if dns_part_size == 0 {
              return false;
            }
            all_num_top_dom = false;
          }
          'a'..='z' | 'A'..='Z' => all_num_top_dom = false,
          _ => return false,
        };
        dash_seen_last = c == '-';
        dns_part_size += 1;
        host_size += 1;
      }
      EmlAddr::End => {
        match c {
          '>' => with_brackets = true,
          '0'..='9' => {
            host_size = 1;
            dns_part_size = 1;
          }
          'a'..='z' | 'A'..='Z' => {
            all_num_top_dom = false;
            host_size = 1;
            dns_part_size = 1;
          }
          '.' => (),
          _ => return false,
        };
        state = EmlAddr::Host;
      }
    }
  }
  let good_size = local_size > 0 && local_size <= MAX_LOCAL_SIZE;
  match state {
    EmlAddr::Name => true,
    EmlAddr::LocalSpaceTest => !dot_seen_last && good_size,
    EmlAddr::LocalQuoteTest => !with_brackets && in_quotes && good_size,
    EmlAddr::Local => !in_quotes && !with_brackets && !dot_seen_last && good_size,
    _ => false,
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::EmailAddress;
  use std::convert::TryFrom;

  #[test]
  fn valid_plain_email_address_is_accepted() {
    assert_valid("l@d");
  }

  #[test]
  fn email_address_must_have_an_arobase() {
    assert_invalid("l.d");
  }

  #[test]
  fn valid_email_address_in_brackets_is_accepted() {
    assert_valid("<l@d>");
  }

  #[test]
  fn valid_email_address_with_name_is_accepted() {
    assert_valid("Some One <l@d>");
  }

  #[test]
  fn email_address_with_name_must_have_brackets() {
    assert_invalid(r#""Some One" l@d"#);
  }

  #[test]
  fn domain_name_may_end_with_dot() {
    assert_valid("l@d.");
  }

  #[test]
  fn bracketted_domain_name_may_end_with_dot() {
    assert_valid("<l@d.>");
  }

  #[test]
  fn domain_name_may_not_start_with_dot() {
    assert_invalid("l@.d");
  }

  #[test]
  fn domain_name_may_not_contain_consecutive_dots() {
    assert_invalid("l@a..z");
  }

  #[test]
  fn domain_name_cannot_be_empty() {
    assert_invalid("l@");
  }

  #[test]
  fn domain_name_may_contain_alphanumeric_dash_and_dot() {
    assert_valid("l@a-z.A-Z.0-9");
  }

  #[test]
  fn domain_name_may_not_contain_other_ascii_chars() {
    assert_invalid("l@a_z.A_Z.0_9");
  }

  #[test]
  fn domain_name_may_not_contain_other_iso_chars() {
    assert_invalid("l@aàz.AéZ.0î9");
  }

  #[test]
  fn domain_name_may_not_contain_other_utf_chars() {
    assert_invalid("l@a…z.A☺️Z.0⩽9");
  }

  #[test]
  fn domain_part_may_not_end_with_dash() {
    assert_invalid("l@a-z.A-.0-9");
  }

  #[test]
  fn domain_part_may_not_start_with_dash() {
    assert_invalid("l@a-z.-Z.0-9");
  }

  #[test]
  fn tld_may_not_be_numeric() {
    assert_invalid("l@a-z.A-Z.09");
  }

  #[test]
  fn dotted_tld_may_not_be_numeric() {
    assert_invalid("l@a-z.A-Z.09.");
  }

  #[test]
  fn bracketted_tld_may_not_be_numeric() {
    assert_invalid("<l@a-z.A-Z.09>");
  }

  #[test]
  fn bracketted_dotted_tld_may_not_be_numeric() {
    assert_invalid("<l@a-z.A-Z.09.>");
  }

  #[test]
  fn domain_part_before_tld_may_be_numeric() {
    assert_valid("l@1.a-z.42.A-Z.0-9");
  }

  #[test]
  fn local_part_can_contain_anything_between_quotes() {
    assert_valid("\"\t\n\r\\\"« 👍 ”\\ŷñΩ\"@d");
  }

  #[test]
  fn local_part_can_contain_any_individually_quoted_character() {
    assert_valid("\\\t\\\n\\\r\\\"\\«\\ \\👍\\ \\”\\\\\\ŷ\\ñ\\Ω@d");
  }

  #[test]
  fn a_trailing_backslash_in_a_quoted_local_part_must_be_quoted() {
    assert_invalid(r#""l\"@d"#);
  }

  #[test]
  fn all_allowed_chars_are_accepted_in_an_unquoted_local_part() {
    assert_valid("!#$%&'*+-/=?^_`.{|}~azAZ09@d");
  }

  #[test]
  fn unquotted_space_is_disallowed_in_local_part() {
    assert_invalid("<l l@d>");
  }

  #[test]
  fn unquotted_quote_is_disallowed_in_local_part() {
    assert_invalid(r#"<l"l@d>"#);
  }

  #[test]
  fn consecutive_dots_are_not_allowed_in_an_unquoted_local_part() {
    assert_invalid("a..z@d");
  }

  #[test]
  fn unquoted_local_part_cannot_start_with_a_dot() {
    assert_invalid(".z@d");
  }

  #[test]
  fn unquoted_local_part_cannot_end_with_a_dot() {
    assert_invalid("a.@d");
  }

  #[test]
  fn unquoted_local_part_may_not_contain_other_ascii_chars() {
    assert_invalid("l\tl@d");
  }

  #[test]
  fn unquoted_local_part_may_not_contain_other_iso_chars() {
    assert_invalid("àéî@d");
  }

  #[test]
  fn unquoted_local_part_may_not_contain_other_utf_chars() {
    assert_invalid("…☺️⩽@d");
  }

  #[test]
  fn local_part_can_start_with_a_quoted_quote() {
    assert_valid("<\\\"l@d>");
  }

  #[test]
  fn local_part_can_start_with_a_quoted_space() {
    assert_valid("<\\ l@d>");
  }

  fn assert_valid(addr: &str) {
    let checked: String = EmailAddress::try_from(addr.to_string()).unwrap().into();
    assert_eq!(addr, &checked);
  }

  fn assert_invalid(addr: &str) {
    assert_eq!(
      Err(format!("Email {} is invalid", addr).into()),
      EmailAddress::try_from(addr.to_string())
    );
  }
}
