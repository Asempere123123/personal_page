use yew::prelude::*;

#[function_component(Languages)]
pub fn home() -> Html {
    html! {
        <>
        <h2 class="text-xl font-thin">{ "Languages" }</h2>
        <table class="table-auto">
            <tr>
                <td class="px-2">{ "Spanish" }</td>
                <td>{ "Nativo" }</td>
            </tr>
            <tr>
                <td class="px-2">{ "English" }</td>
                <td>{ "B2" }</td>
            </tr>
            <tr>
                <td class="px-2">{ "German" }</td>
                <td>{ "B2(por ahora B1)" }</td>
            </tr>
            <tr>
                <td class="px-2">{ "Basque" }</td>
                <td>{ "Basic" }</td>
            </tr>
        </table>
        </>
    }
}
