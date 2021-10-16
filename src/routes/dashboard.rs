use askama::Template;

use crate::request_guards::LoggedInUser;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {}

#[get("/dashboard")]
pub fn dashboard(_user: LoggedInUser) -> DashboardTemplate {
    DashboardTemplate {}
}
