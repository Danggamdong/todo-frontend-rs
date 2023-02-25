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
        use_effect_with_deps(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_todos: Vec<Todo> = Request::get("/Todolist?name=TestUser")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                state.set(State { todos: fetched_todos });
                println!("Hello");
            });
            || ()
        }, ());
    }

    let onadd = {
        let state = state.clone();
        Callback::from(move |description: String| {
            let mut todos = state.todos.clone();
            todos.push(Todo {id: todos.last().map(|todo| todo.id + 1).unwrap_or(1), description});
            state.set(State { todos });
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
