use common::traits::{
    ToClass,
    ToStyle
};

#[derive(Clone, Copy, PartialEq)]
pub enum Style {
    Color(Color)
}

impl ToStyle for Style {
    fn to_style(&self) -> &'static str {
        match &self {
            Style::Color(c) => c.to_style()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger
}

impl Color {
    pub fn to_text_class(&self) -> &'static str {
        match &self {
            Color::Primary =>   "has-text-primary",
            Color::Link =>      "has-text-link",
            Color::Info =>      "has-text-info",
            Color::Success =>   "has-text-success",
            Color::Warning =>   "has-text-warning",
            Color::Danger =>    "has-text-danger",
        }
    }
}

impl ToStyle for Color {
    fn to_style(&self) -> &'static str {
        match &self {
            Color::Primary =>   "color: hsl(171, 100%, 41%)",
            Color::Link =>      "color: hsl(217, 71%, 53%)",
            Color::Info =>      "color: hsl(204, 86%, 53%)",
            Color::Success =>   "color: hsl(141, 53%, 53%)",
            Color::Warning =>   "color: hsl(48, 100%, 67%)",
            Color::Danger =>    "color: hsl(348, 100%, 61%)"
        }
    }
}

impl ToClass for Color {
    fn to_class(&self) -> &'static str {
        match self {
            Color::Primary =>   "is-primary",
            Color::Link =>      "is-link",
            Color::Info =>      "is-info",
            Color::Success =>   "is-success",
            Color::Warning =>   "is-warning",
            Color::Danger =>    "is-danger",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl ToClass for Size {
    fn to_class(&self) -> &'static str {
        match self {
            Size::Small =>   "is-small",
            Size::Normal =>   "is-normal",
            Size::Medium =>   "is-medium",
            Size::Large =>   "is-large",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Styles {
    Rounded,
    Loading,
    Outlined
}

impl ToClass for Styles {
    fn to_class(&self) -> &'static str {
        match self {
            Styles::Rounded => "is-rounded",
            Styles::Loading => "is-loading",
            Styles::Outlined => "is-outlined"
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum States {
    Hover,
    Focus,
    Loading
}

impl ToClass for States {
    fn to_class(&self) -> &'static str {
        match self {
            States::Hover => "is-hovered",
            States::Focus => "is-focused",
            States::Loading => "is-loading",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Position {
    Left,
    Right
}

impl ToClass for Position {
    fn to_class(&self) -> &'static str {
        match &self {
            Position::Left => "is-left",
            Position::Right => "is-right"
        }
    }
}