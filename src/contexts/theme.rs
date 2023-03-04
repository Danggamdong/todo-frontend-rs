use std::ops::Deref;

use once_cell::sync::Lazy;
use stylist::yew::*;
use yew::prelude::*;

pub struct Theme {
    pub background_color: String,
    pub font_color: String,
    pub paper_color: String,
}

#[derive(Clone, PartialEq)]
pub enum ThemeKind {
    Dark,
    Light,
}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "white".to_string(),
            background_color: "black".to_string(),
            paper_color: "rgb(50, 50, 50)".to_string(),
        });

        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "black".to_string(),
            background_color: "rgb(237, 244, 255)".to_string(),
            paper_color: "white".to_string(),
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            ThemeKind::Light => &LIGHT_THEME,
        }
    }
}

#[derive(Clone)]
pub struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        Self { inner }
    }

    pub fn set(&self, kind: ThemeKind) {
        self.inner.set(kind)
    }

    pub fn kind(&self) -> ThemeKind {
        (*self.inner).clone()
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        self.inner.current()
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(PartialEq, Properties)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme_kind = use_state(|| ThemeKind::Light);

    let theme_ctx = ThemeContext::new(theme_kind);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}
