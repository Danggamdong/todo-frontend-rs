use chrono::prelude::*;
use gloo_net::http::Request;
use stylist::yew::*;
use yew::prelude::*;

mod components;
mod contexts;
mod state;

use components::entry::Entry;
use components::header_input::HeaderInput;
use contexts::theme::{ThemeProvider, use_theme};
use state::{State, Todo};

#[styled_component]
fn Main() -> Html {
    let state = use_state(|| State { todos: vec![] });
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("/todos")
                        .send()
                        .await
                        .expect("Failed to request");

                    let fetched_todos: Vec<Todo> = if response.ok() {
                        response.json().await.expect("Failed to serialize `Todo`")
                    } else {
                        panic!("Failed to get response")
                    };

                    state.set(State {todos: fetched_todos});
                });
                || ()
            },
            (),
        );
    }

    let onadd = {
        let state = state.clone();
        Callback::from(move |description: String| {
            let mut todos = state.todos.clone();
            let todo = Todo {
                id: todos.last().map(|todo| todo.id + 1).unwrap_or(1),
                description: description,
                created_at: Utc::now().timestamp(),
                is_finished: false,
            };
            todos.push(todo.clone());
            state.set(State { todos });

            wasm_bindgen_futures::spawn_local(async move {
                Request::post("/todos").header("Content-Type", "application/json").body(todo.to_js_value());
            });
        })
    };

    let onremove = {
        let state = state.clone();
        Callback::from(move |id: usize| {
            let mut todos = state.todos.clone();
            todos.retain(|todo| todo.id != id);
            state.set(State { todos });
        })
    };

    html! {
        <div id="todo-main">
            <div id="header">
                <h1>{ "todos" }</h1>
                <HeaderInput {onadd} />
            </div>
            <div id="main">
                <ul id="todo-list">
                    {for state.todos.iter().cloned().map(|todo| html! {<Entry {todo} onremove={onremove.clone()}/>})}
                </ul>
            </div>
            <div id="footer">
            </div>
        </div>
    }
}

#[styled_component]
fn App() -> Html {
    let theme = use_theme();

    html! {
        <>
            <Global css={css!(
                r#"
                    html {
                        font-family: sans-serif;

                        padding: 0;
                        margin: 0;

                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;

                        background-color: ${bg};
                        color: ${ft_color};
                    }
                "#,
                bg = theme.background_color.clone(),
                ft_color = theme.font_color.clone(),
            )} />
            <header>
            </header>
            <section class={css!(
                r#"
                    box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                    height: 500px;
                    width: 500px;
                    border-radius: 5px;
                    display: flex;
                    justify-content: space-around;
                    align-items: center;
                    padding: 15px;
                    box-sizing: border-box;
                    flex-direction: column;
                    background-color: ${bg};
                "#,
                bg = theme.paper_color.clone()
            )} id="todo-wrapper">
                <Main />
            </section>
        </>
    }
}

#[styled_component]
pub fn Root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
