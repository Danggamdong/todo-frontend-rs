use chrono::prelude::*;
use gloo_net::http::Request;
use yew::prelude::*;

mod components;
mod state;

use components::entry::Entry;
use components::header_input::HeaderInput;
use state::{State, Todo};

#[function_component]
fn App() -> Html {
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
        <div class="todo-wrapper">
            <section class="todo-app">
                <header class="header">
                    <h1>{ "todos" }</h1>
                    <HeaderInput {onadd} />
                </header>
                <section class="main">
                    <ul class="todo-list">
                        {for state.todos.iter().cloned().map(|todo| html! {<Entry {todo} onremove={onremove.clone()}/>})}
                    </ul>
                </section>
                <footer class="footer">
                </footer>
            </section>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
