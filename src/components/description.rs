use yew::prelude::*;

#[function_component(Description)]
pub fn home() -> Html {
    html! {
        <>
        <h2 class="text-xl font-thin">{ "Description" }</h2>
        <ul class="pl-2">
            <li>{ "• hola" }</li>
            <li>{ "• hola" }</li>
        </ul>
        </>
    }
}
