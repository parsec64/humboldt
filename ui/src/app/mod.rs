mod app;

#[derive(Debug)]
pub struct App {
    theme: AppTheme,
}

#[derive(Debug)]
pub enum AppTheme {
    Dark,
    Light,
}

#[derive(Debug)]
pub enum AppMessage {}
