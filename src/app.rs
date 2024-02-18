use leptos::*;

#[component]
pub fn App(initial_value: i32) -> impl IntoView {
    let (counter, set_counter) = create_signal(initial_value);
    view! {
        <div>
            <button class="clear-button" on:click = move |_| set_counter.set(0)>"Clear"</button><br/>
            <button on:click = move |_| set_counter.update(|counter| *counter += 1)>"Increment"</button><br/>
            <span>"Counter: " {counter} </span><br/>
            <button on:click = move |_| set_counter.update(|counter| *counter -= 1)>"Decrement"</button>
        </div>
    }
}
