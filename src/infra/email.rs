use crate::domain::{Config, EmailAddress, EmailData, EmailPort, Value};
use chrono::Utc;
use lettre_email::{mime::Mime, Header, MimeMultipartType, PartBuilder};
use std::{
  collections::HashMap,
  convert::TryFrom,
  ffi::OsString,
  io::Write,
  process::{Command, Stdio},
  str::FromStr,
};

const DEFAULT_FROM: &str = "pyruse@localhost";
const DEFAULT_TO: &str = "hostmaster@localhost";
const DEFAULT_SENDMAIL: &[&str] = &["/usr/bin/sendmail", "-t"];

#[derive(Clone)]
pub struct ProcessEmailAdapter {
  from: EmailAddress,
  to: Vec<EmailAddress>,
  sendmail_cmd: OsString,
  sendmail_params: Vec<OsString>,
}

impl ProcessEmailAdapter {
  pub fn new(conf: &Config) -> ProcessEmailAdapter {
    let mut default_options = HashMap::new();
    default_options.insert("from".to_string(), Value::Str(DEFAULT_FROM.to_string()));
    default_options.insert(
      "to".to_string(),
      Value::List(vec![Value::Str(DEFAULT_TO.to_string())]),
    );
    default_options.insert(
      "sendmail".to_string(),
      Value::List(
        DEFAULT_SENDMAIL
          .iter()
          .map(|s| Value::Str(s.to_string()))
          .collect(),
      ),
    );
    let options = match conf.options.get("email") {
      Some(Value::Map(m)) => m,
      _ => &default_options,
    };
    let from = match options.get("from") {
      Some(Value::Str(s)) => {
        EmailAddress::try_from(s.clone()).expect(&format!("Invalid address: {}", s))
      }
      _ => EmailAddress::try_from(DEFAULT_FROM.to_string()).unwrap(),
    };
    let mut to = match options.get("to") {
      Some(Value::Str(s)) => vec!(EmailAddress::try_from(s.clone()).expect(&format!("Invalid address: {}", s))),
      Some(Value::List(l)) => l.iter().map(|v| match v {
        Value::Str(s) => EmailAddress::try_from(s.clone()).expect(&format!("Invalid address: {}", s)),
        _ => panic!("One item in the list found in property “to” of email options is not an email address as expected"),
      }).collect(),
      _ => vec!(EmailAddress::try_from(DEFAULT_TO.to_string()).unwrap()),
    };
    if to.is_empty() {
      to = vec![EmailAddress::try_from(DEFAULT_TO.to_string()).unwrap()];
    }
    let mut sendmail_params = match options.get("sendmail") {
      Some(Value::Str(s)) => vec!(OsString::from_str(s).expect(&format!("“{}” could not be read", s))),
      Some(Value::List(l)) => l.iter().map(|v| match v {
        Value::Str(s) => OsString::from_str(s).expect(&format!("“{}” could not be read", s)),
        _ => panic!("One item in the “sendmail” command-line components of email options is not a shell command or a parameter"),
      }).collect(),
      _ => DEFAULT_SENDMAIL
      .iter()
      .map(|s| OsString::from_str(s).unwrap())
      .collect(),
    };
    if sendmail_params.is_empty() {
      sendmail_params = DEFAULT_SENDMAIL
        .iter()
        .map(|s| OsString::from_str(s).unwrap())
        .collect();
    }
    let sendmail_cmd = sendmail_params.remove(0);
    ProcessEmailAdapter {
      from,
      to,
      sendmail_cmd,
      sendmail_params,
    }
  }
}

impl EmailPort for ProcessEmailAdapter {
  fn send(&mut self, email: EmailData) -> Result<(), ()> {
    let mut main_part = PartBuilder::new()
      .header(Header::new("From".to_string(), self.from.to_string()))
      .header(Header::new(
        "Return-Path".to_string(),
        self.from.to_string(),
      ))
      .header(Header::new("Date".to_string(), Utc::now().to_rfc2822()))
      .header(Header::new(
        "To".to_string(),
        self
          .to
          .iter()
          .map(|m| m.to_string())
          .collect::<Vec<String>>()
          .join(","),
      ))
      .header(Header::new(
        "Subject".to_string(),
        format!(
          "=?utf-8?Q?{}?=",
          quoted_printable::encode_to_str(email.subject)
        ),
      ))
      .message_type(MimeMultipartType::Alternative);
    if let Some(text) = email.text {
      main_part = main_part.child(
        PartBuilder::new()
          .header(Header::new(
            "Content-Transfer-Encoding".to_string(),
            "QUOTED-PRINTABLE".to_string(),
          ))
          .body(quoted_printable::encode_to_str(text))
          .content_type(&Mime::from_str("text/plain; charset=UTF-8").unwrap())
          .build(),
      );
    }
    if let Some(html) = email.html {
      main_part = main_part.child(
        PartBuilder::new()
          .header(Header::new(
            "Content-Transfer-Encoding".to_string(),
            "QUOTED-PRINTABLE".to_string(),
          ))
          .body(quoted_printable::encode_to_str(html))
          .content_type(&Mime::from_str("text/html; charset=UTF-8").unwrap())
          .build(),
      );
    }
    let mime_message = main_part.build().as_string();
    let mut sendmail = Command::new(&self.sendmail_cmd);
    for p in &self.sendmail_params {
      sendmail.arg(p);
    }
    match sendmail.stdin(Stdio::piped()).spawn() {
      Ok(mut sendmail_process) => match sendmail_process.stdin.as_mut() {
        Some(stdin) => match stdin.write_all(mime_message.as_bytes()) {
          Ok(_) => Ok(()),
          Err(_) => Err(()),
        },
        None => Err(()),
      },
      Err(_) => Err(()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{ProcessEmailAdapter, DEFAULT_FROM, DEFAULT_SENDMAIL, DEFAULT_TO};
  use crate::domain::{Config, EmailData, EmailPort, Value};
  use indexmap::IndexMap;
  use regex::Regex;
  use std::collections::HashMap;
  use std::env::temp_dir;
  use std::fs;

  #[test]
  fn if_no_email_options_then_defaults_are_used() {
    let conf = Config {
      actions: IndexMap::new(),
      options: HashMap::new(),
    };
    let proc = ProcessEmailAdapter::new(&conf);
    let from: String = proc.from.clone().into();
    let to: String = proc.to[0].clone().into();
    let sendmail = &[
      proc.sendmail_cmd.to_str().unwrap(),
      proc.sendmail_params[0].to_str().unwrap(),
    ];
    assert_eq!(DEFAULT_FROM.to_string(), from);
    assert_eq!(DEFAULT_TO.to_string(), to);
    assert_eq!(DEFAULT_SENDMAIL, sendmail);
  }

  #[test]
  fn if_no_from_email_then_default_from() {
    let conf = test_config("", |o| {
      o.remove("from");
    });
    let proc = ProcessEmailAdapter::new(&conf);
    let from: String = proc.from.clone().into();
    assert_eq!(DEFAULT_FROM.to_string(), from);
  }

  #[test]
  fn from_email_must_be_a_string_or_default_from_is_used() {
    let conf = test_config("", |o| {
      o.insert("from".to_string(), Value::Int(33));
    });
    let proc = ProcessEmailAdapter::new(&conf);
    let from: String = proc.from.clone().into();
    assert_eq!(DEFAULT_FROM.to_string(), from);
  }

  #[test]
  #[should_panic(expected = "Invalid address: wrong 😱")]
  fn an_email_address_must_be_valid() {
    let conf = test_config("", |o| {
      o.insert("from".to_string(), Value::Str("wrong 😱".to_string()));
    });
    ProcessEmailAdapter::new(&conf);
  }

  #[test]
  fn if_no_to_emails_then_default_to_is_used() {
    let conf = test_config("", |o| {
      o.remove("to");
    });
    let proc = ProcessEmailAdapter::new(&conf);
    let to: String = proc.to[0].clone().into();
    assert_eq!(DEFAULT_TO.to_string(), to);
  }

  #[test]
  #[should_panic(
    expected = "One item in the list found in property “to” of email options is not an email address as expected"
  )]
  fn to_emails_must_be_strings() {
    let conf = test_config("", |o| {
      o.insert(
        "to".to_string(),
        Value::List(vec![
          Value::Str("ok@example.org".to_string()),
          Value::Bool(true),
        ]),
      );
    });
    ProcessEmailAdapter::new(&conf);
  }

  #[test]
  fn if_empty_to_list_then_default_to_is_used() {
    let conf = test_config("", |o| {
      o.insert("to".to_string(), Value::List(Vec::new()));
    });
    let proc = ProcessEmailAdapter::new(&conf);
    let to: String = proc.to[0].clone().into();
    assert_eq!(DEFAULT_TO.to_string(), to);
  }

  #[test]
  fn if_no_sendmail_command_then_default_sendmail_is_used() {
    let conf = test_config("", |o| {
      o.remove("sendmail");
    });
    let proc = ProcessEmailAdapter::new(&conf);
    let sendmail = &[
      proc.sendmail_cmd.to_str().unwrap(),
      proc.sendmail_params[0].to_str().unwrap(),
    ];
    assert_eq!(DEFAULT_SENDMAIL, sendmail);
  }

  #[test]
  fn sendmail_command_cannot_be_an_empty_list_or_default_sendmail_is_used() {
    let conf = test_config("", |o| {
      o.insert("sendmail".to_string(), Value::List(Vec::new()));
    });
    let proc = ProcessEmailAdapter::new(&conf);
    let sendmail = &[
      proc.sendmail_cmd.to_str().unwrap(),
      proc.sendmail_params[0].to_str().unwrap(),
    ];
    assert_eq!(DEFAULT_SENDMAIL, sendmail);
  }

  #[test]
  fn adapter_gets_created_if_all_options_are_correct() {
    let conf = test_config("", |_| {});
    ProcessEmailAdapter::new(&conf);
  }

  #[test]
  fn generate_and_check_rfc_compliant_email() {
    let mut temp = temp_dir();
    temp.push("pyruse-test.eml");
    let filename = temp.to_str().unwrap();
    let conf = test_config(filename, |_| {});
    let mut proc = ProcessEmailAdapter::new(&conf);
    let data = EmailData {
      text: Some("= Flags\n\n« 🇫🇷🇨🇦🇯🇵 »".to_string()),
      html: Some(
        r#"<html>
        <body>
          <h1>Flags</h1>
          <p>« 🇫🇷🇨🇦🇯🇵 »</p>
        </body>
      </html>"#
          .to_string(),
      ),
      subject: "Ého ! Ça va ? … 😼".to_string(),
    };
    proc.send(data).unwrap();

    let mut regex_str = r"(?i)\A(?:".to_string();
    regex_str.push_str(&vec!(
      r"From: pyruse@localhost\r\n",
      r"Return-Path: pyruse@localhost\r\n",
      r"Date: \w+\.?, \d{1,2} \w+\.? \d{4} \d{1,2}:\d{2}:\d{2} [\-+]\d{4}\r\n",
      r"To: root@localhost,abuse@localhost\r\n",
      r"Subject: =\?utf-8\?Q\?=C3=89ho=E2=80=AF! =C3=87a va=E2=80=AF\? =E2=80=A6\r\n\s+=F0=9F=98=BC\?=\r\n",
      r"Content-Type: multipart/alternative; boundary=(.*)\r\n",
    ).iter()
    .map(|s| s.clone())
    .collect::<Vec<&str>>()
    .join("|"));
    regex_str.push_str(r"){6}(?:(?:\r\n)+--.*\r\n(?:");
    regex_str.push_str(
      &vec![
        r"Content-Transfer-Encoding: QUOTED-PRINTABLE\r\n",
        r"Content-Type: text/plain; charset=utf-8\r\n",
      ]
      .iter()
      .map(|s| s.clone())
      .collect::<Vec<&str>>()
      .join("|"),
    );
    regex_str.push_str(r"){2}(?:\r\n)+");
    regex_str.push_str(
      r"=3D Flags=0A=0A=C2=AB=C2=A0=F0=9F=87=AB=F0=9F=87=B7=F0=9F=87=A8=F0=9F=87=A6=\r\n=F0=9F=87=AF=F0=9F=87=B5=C2=A0=C2=BB\r\n",
    );
    regex_str.push_str(r"|(?:\r\n)+--.*\r\n(?:");
    regex_str.push_str(
      &vec![
        r"Content-Transfer-Encoding: QUOTED-PRINTABLE\r\n",
        r"Content-Type: text/html; charset=utf-8\r\n",
      ]
      .iter()
      .map(|s| s.clone())
      .collect::<Vec<&str>>()
      .join("|"),
    );
    regex_str.push_str(r"){2}(?:\r\n)+");
    regex_str.push_str(r"<html>=0A        <body>=0A          <h1>Flags</h1>=0A          <p>=C2=AB=C2=\r\n=A0=F0=9F=87=AB=F0=9F=87=B7=F0=9F=87=A8=F0=9F=87=A6=F0=9F=87=AF=F0=9F=87=B5=\r\n=C2=A0=C2=BB</p>=0A        </body>=0A      </html>\r\n");
    regex_str.push_str(r"){2}(?:\r\n)*--.*--(?:\r\n)*\z");
    let expect_regex = Regex::new(&regex_str).unwrap();

    let file = fs::read_to_string(filename).unwrap();
    //println!("REGEX = [[{}]]\n", &regex_str);
    //println!("FILE = [[{}]]\n", &file);
    assert!(expect_regex.is_match(&file));
    fs::remove_file(filename).unwrap();
  }

  fn test_config(test_file: &str, alter_email_options: fn(&mut HashMap<String, Value>)) -> Config {
    let mut sendmail_opts = HashMap::new();
    sendmail_opts.insert(
      "from".to_string(),
      Value::Str("pyruse@localhost".to_string()),
    );
    sendmail_opts.insert(
      "to".to_string(),
      Value::List(
        vec!["root@localhost", "abuse@localhost"]
          .iter()
          .map(|s| Value::Str(s.to_string()))
          .collect(),
      ),
    );
    sendmail_opts.insert(
      "sendmail".to_string(),
      Value::List(
        //vec!["bash", "-c", "tee test.eml | sendmail -t"]
        vec!["bash", "-c", &format!(r#"cat >"{}""#, test_file)]
          .iter()
          .map(|s| Value::Str(s.to_string()))
          .collect(),
      ),
    );
    alter_email_options(&mut sendmail_opts);
    let mut options = HashMap::new();
    options.insert("email".to_string(), Value::Map(sendmail_opts));
    Config {
      actions: IndexMap::new(),
      options,
    }
  }
}
