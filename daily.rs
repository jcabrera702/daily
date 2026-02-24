use yew::prelude::*;
use std::collections::HashMap;

const MS_PER_24H: i64 = 24 * 60 * 60 * 1000;

fn now() -> i64 {
    js_sys::Date::now() as i64
}

#[derive(Clone)]
struct Task {
    id: i64,
    name: String,
    last_done: i64,
    last_done_by: String,
    notes: String,
}

fn health_percent(last_done: i64) -> i64 {
    let elapsed = now() - last_done;
    (100 - (elapsed * 100 / MS_PER_24H)).max(0)
}

#[function_component(App)]
fn app() -> Html {
    let tasks = use_state(|| HashMap::<i64, Task>::new());
    let input_value = use_state(|| "".to_string());
    let username = "Unknown".to_string();

    let add_task = {
        let tasks = tasks.clone();
        let input_value = input_value.clone();
        let username = username.clone();

        Callback::from(move |_| {
            let name = (*input_value).trim().to_string();
            if name.is_empty() { return; }

            let id = now();
            let mut new_map = (*tasks).clone();
            new_map.insert(id, Task {
                id,
                name,
                last_done: now(),
                last_done_by: username.clone(),
                notes: "".into(),
            });
            tasks.set(new_map);
            input_value.set("".into());
        })
    };

    let delete_task = {
        let tasks = tasks.clone();
        Callback::from(move |id: i64| {
            let mut new_map = (*tasks).clone();
            new_map.remove(&id);
            tasks.set(new_map);
        })
    };

    let complete_task = {
        let tasks = tasks.clone();
        let username = username.clone();
        Callback::from(move |id: i64| {
            let mut new_map = (*tasks).clone();
            if let Some(mut t) = new_map.get_mut(&id) {
                t.last_done = now();
                t.last_done_by = username.clone();
            }
            tasks.set(new_map);
        })
    };

    html! {
        <div style="padding:20px;">
            <h1>{ "Shared Daily Tasks (Rust+Yew)" }</h1>

            <input
                value={(*input_value).clone()}
                oninput={{
                    let input_value = input_value.clone();
                    Callback::from(move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        input_value.set(input.value());
                    })
                }}
                placeholder="New task..."
            />
            <button onclick={add_task}>{ "Add" }</button>

            <div style="margin-top:20px;">
                {
                    for tasks.values().map(|task|{
                        let hp = health_percent(task.last_done);
                        let delete_task = delete_task.clone();
                        let complete_task = complete_task.clone();
                        html!{
                            <div style="border:1px solid gray;margin-bottom:10px;padding:10px;">
                                <b>{ &task.name }</b><br/>
                                { format!("Health: {}%", hp) }<br/>
                                { format!("Last done by {} at {}", task.last_done_by, task.last_done) }<br/>

                                <button onclick={{let id=task.id; Callback::from(move |_| complete_task.emit(id))}}>
                                    { if hp<=0 {"Wake Up Task"} else {"Complete"} }
                                </button>

                                <button onclick={{let id=task.id; Callback::from(move |_| delete_task.emit(id))}}>
                                    { "Delete" }
                                </button>
                            </div>
                        }
                    })
                }
            </div>
        </div>
    }
}
