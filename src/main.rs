use yew::prelude::*;
use yew_hooks::use_async;

#[function_component(App)]
fn app() -> Html {
  let date = garf::random_date();

  let state = use_async( garf::get_comic_url(date) );

  let onclick = {
    let state = state.clone();
    Callback::from(move |_| {
      state.run();
    })
  };

  html! {
    <>
      <h1>{ "Garf" }</h1>
      <p>{ date }</p>
      <button {onclick}>{ "Run" }</button>
      <p>
        {
          if state.loading {
            html! { "Loading..." }
          } else {
            html! { "Not loading." }
          }
        }
      </p>
      <p>
        {
          if let Some(error) = &state.error {
            html! { error }
          } else {
            html! { "No error." }
          }
        }
      </p>
      <p>
        {
          if let Some(data) = &state.data {
            html! { 
              <>
                { data }
                <img src={ data.to_owned() } />
              </>
            }
          } else {
            html! { "No result." }
          }
        }
      </p>
    </>
  }
}

fn main() {
  yew::start_app::<App>();
}
