use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct HeaderInputProps {
    pub onadd: Callback<String>,
}

#[function_component]
pub fn HeaderInput(props: &HeaderInputProps) -> Html {
    let onkeypress = {
        let onadd = props.onadd.clone();
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                onadd.emit(value);
            }
        }
    };

    html! {
        <div>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
        </div>
    }
}
