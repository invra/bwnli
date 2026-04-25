use yew::prelude::*;

#[component]
pub fn Counter() -> Html {
    let counter = use_state(|| 0);
    let increment = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let decrement = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! {
        <div class="min-h-screen flex flex-col gap-6 justify-center items-center">
            <p class="text-center text-6xl">{ *counter }</p>
            <div class="flex flex-col gap-1">
                <button class="bg-blue-500 py-2 px-5 min-w-60 text-gray-200 rounded-md" onclick={decrement}>{ "-" }</button>
                <button class="bg-blue-500 py-2 px-5 min-w-60 text-gray-200 rounded-md" onclick={increment}>{ "+" }</button>
            </div>
        </div>
    }
}
