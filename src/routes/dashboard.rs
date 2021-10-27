use askama::Template;
use rocket::form::Form;
use rocket::response::Redirect;

use crate::cookies::Session;
use crate::db::Transaction;
use crate::routes;
use crate::schemas::users::User;
use crate::{basics::*, csrf};

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    csrf_token: String,
}

#[get("/dashboard")]
pub async fn dashboard(
    _user: User,
    session: Session,
) -> Result<DashboardTemplate, AppError> {
    Ok(DashboardTemplate {
        csrf_token: session.csrf_token,
    })
}

#[get("/dashboard", rank = 1)]
pub fn not_logged_in() -> Redirect {
    Redirect::found(uri!(routes::index::index()))
}

#[post("/dashboard/delete_account", data = "<form>")]
pub async fn delete_account(
    user: User,
    mut tx: Transaction,
    csrf: crate::csrf::Validation,
    form: Form<csrf::Form>,
) -> Result<Redirect, AppError> {
    csrf.validate(&form.csrf_token)?;

    crate::storage::users::delete_user(&mut tx, user).await?;

    tx.0.commit().await?;

    Ok(Redirect::to(uri!(routes::index::index())))
}
