mod app;
use app::App;
use leptos::*;

fn main() {
    mount_to_body( || {
        view! {
            <App initial_value = 0 />
        }
    })
}
