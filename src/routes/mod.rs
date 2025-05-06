mod admin;
mod health_check;
mod home;
mod login;
mod subscriptions;
mod subscriptions_confirm;

pub use admin::{
    admin_dashboard, change_password, change_password_form, log_out, publish_newsletter,
    publish_newsletter_form,
};
pub use health_check::health_check;
pub use home::home;
pub use login::{login, login_form};
pub use subscriptions::subscribe;
pub use subscriptions_confirm::confirm;
