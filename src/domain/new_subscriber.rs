use actix_web::web;

use crate::routes::FormData;

use super::{SubscriberEmail, SubscriberName};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl TryFrom<web::Form<FormData>> for NewSubscriber {
    type Error = String;
    fn try_from(form: web::Form<FormData>) -> Result<Self, Self::Error> {
        Ok(NewSubscriber {
            email: form.0.email.try_into()?,
            name: form.0.name.try_into()?,
        })
    }
}
