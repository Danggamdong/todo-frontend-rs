use yew::prelude::*;

use crate::state::Todo;

#[derive(Clone, PartialEq, Properties)]
pub struct EntryProps {
    pub todo: Todo,
    pub onremove: Callback<usize>,
}

#[function_component]
pub fn Entry(props: &EntryProps) -> Html {
    let id = props.todo.id;
    let classes = Classes::from("todo");

    let onremove = {
        let onremove = props.onremove.clone();
        move |_| onremove.emit(id)
    };

    html! {
        <li {classes}>
            <div class="entry">
                <label>{ &props.todo.description }</label>
                <button class="destroy" onclick={onremove}>{"Delete"}</button>
            </div>
        </li>
    }
}
