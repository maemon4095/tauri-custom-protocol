use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen(inline_js = r#"
export async function fetchWithBinaryBody(url, body) {
    return await fetch(url, {
        method: "POST",
        body
    });
}
"#)]
extern "C" {

    #[wasm_bindgen(js_name=fetchWithBinaryBody)]
    async fn fetch(url: &str, body: &[u8]) -> JsValue;

}

async fn sample_binary_method(input: Vec<usize>) -> String {
    #[derive(serde::Serialize)]
    struct Args(Vec<usize>);

    let bin = bincode::serialize(&Args(input)).unwrap();

    let response = fetch(
        "https://mybinary.localhost?method=sample_binary_method",
        &bin,
    )
    .await;
    let response: web_sys::Response = response.dyn_into().unwrap();

    let buffer = response.array_buffer().expect("invalid response");
    let buffer = wasm_bindgen_futures::JsFuture::from(buffer)
        .await
        .expect("invalid response");

    let array = js_sys::Uint8Array::new(&buffer);

    let vec = array.to_vec();

    let ret: String = bincode::deserialize(&vec).unwrap();

    ret
}

#[function_component(App)]
pub fn app() -> Html {
    let result = use_state(|| String::new());
    let onclick = Callback::from({
        let result = result.clone();
        move |_| {
            let result = result.clone();
            spawn_local(async move {
                let val = sample_binary_method(vec![1, 2, 3, 4, 5]).await;

                result.set(val);
            })
        }
    });

    html! {
        <main class="container">
            <button {onclick}>{"call sample_binary_method"}</button>
            <p>{&*result}</p>
        </main>
    }
}
