use yew::prelude::*;

use crate::components::{Contact, Description, Languages};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container px-32 mt-16 grid grid-cols-3">
            <div class="col-span-1">
                <img class="pb-1" src="data:image/jpeg;base64,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" />
                <h1 class="text-2xl font-bold">{ "Alejandro Sempere" }</h1>
                <h2 class="text-xl font-thin">{ "Estudiante" }</h2>

                <Contact />

            </div>
            <div class="col-span-2 grid grid-cols-2">
                <div class="col-span-1">
                    <Description />
                    <Languages />
                </div>
                <div class="col-span-1">
                    <h1>{ "holasass" }</h1>
                </div>

            </div>
        </div>
    }
}
