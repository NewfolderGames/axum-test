use handlebars::Handlebars;

#[derive(Clone)]
pub struct AppState<'a> {
    pub handlebars: Handlebars<'a>
}
