use crate::domain::{
  Action, EmailData, EmailPort, Error, ModuleArgs, Record, Singleton, Template, Value,
};
use crate::singleton_borrow;

pub struct Email {
  mailer: Singleton<dyn EmailPort>,
  email: EmailData,
  template: Template,
}

impl Email {
  pub fn from_args(mut args: ModuleArgs, mailer: Singleton<dyn EmailPort>) -> Email {
    let subject = match args.remove("subject") {
      Some(Value::Str(s)) => s,
      _ => "Pyruse Notification".into(),
    };
    let email = EmailData::new(subject);
    let template = match args.remove("message") {
      Some(Value::Str(s)) => Template::new(s),
      _ => panic!("The Email action needs a message template in “message”"),
    };
    Email {
      mailer,
      email,
      template,
    }
  }

  fn clone_email(&self, record: &Record) -> EmailData {
    let mut email = self.email.clone();
    email.text = Some(self.template.format(record));
    email
  }
}

impl Action for Email {
  fn act(&mut self, record: &mut Record) -> Result<(), Error> {
    singleton_borrow!(self.mailer).send(self.clone_email(record))
  }
}
