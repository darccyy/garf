use chrono::Duration;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount};

#[function_component(App)]
fn app() -> Html {
  // Set dates for today and random
  //? Remove
  let first = garf::first_date();
  let random = garf::random_date();
  let today = garf::today_date();

  // Create date state
  let date = use_state(|| random);

  // Create async fetch hook
  let state = use_async(garf::get_comic_url(*date));
  // Run if initial component mount (page load)
  if use_is_first_mount() {
    state.run();
  }

  // State of if new image has finished loading
  //TODO Set `false` when new image loads
  let image_loaded = use_state(|| false);
  let onload_image = {
    let image_loaded = image_loaded.clone();
    Callback::from(move |_| {
      image_loaded.set(true);
    })
  };

  // Create onclick event for first date button
  let onclick_first = {
    let date = date.clone();
    let state = state.clone();
    let image_loaded = image_loaded.clone();
    Callback::from(move |_| {
      date.set(first);
      state.run();
      image_loaded.set(false);
    })
  };
  // Create onclick event for random button
  let onclick_random = {
    let date = date.clone();
    let state = state.clone();
    let image_loaded = image_loaded.clone();
    Callback::from(move |_| {
      date.set(garf::random_date());
      state.run();
      image_loaded.set(false);
    })
  };
  // Create onclick event for today button
  let onclick_today = {
    let date = date.clone();
    let state = state.clone();
    let image_loaded = image_loaded.clone();
    Callback::from(move |_| {
      date.set(today);
      state.run();
      image_loaded.set(false);
    })
  };

  // Create onclick event for previous button
  let onclick_prev = {
    let date = date.clone();
    let state = state.clone();
    let image_loaded = image_loaded.clone();
    Callback::from(move |_| {
      date.set(*date - Duration::days(1));
      state.run();
      image_loaded.set(false);
    })
  };
  // Create onclick event for next button
  let onclick_next = {
    let date = date.clone();
    let state = state.clone();
    let image_loaded = image_loaded.clone();
    Callback::from(move |_| {
      date.set(*date + Duration::days(1));
      state.run();
      image_loaded.set(false);
    })
  };

  // If current date is first ever date, or today, respectively
  let is_disabled_prev = *date == first;
  let is_disabled_next = *date == today;

  // Format date as string for <input/>
  let date_str = garf::date_to_string(*date, "-", true);

  // When date input changes
  let onchange_input = {
    let date = date;
    let state = state.clone();
    let image_loaded = image_loaded.clone();
    Callback::from(move |e: Event| {
      let target: HtmlInputElement = e.target_unchecked_into();
      date.set(
        garf::input_string_to_date(&target.value()).expect("Input date not properly formatted"),
      );
      state.run();
      image_loaded.set(false);
    })
  };

  html! {
    <>
      <h1>{ "Garf" }</h1>

      <input type="date" value={ date_str } onchange={onchange_input} />
      <button onclick={onclick_first}>{ "First" }</button>
      <button onclick={onclick_random}>{ "Random Date" }</button>
      <button onclick={onclick_today}>{ "Today" }</button>

      <button onclick={onclick_prev} disabled={is_disabled_prev}>{ "<" }</button>
      <button onclick={onclick_next} disabled={is_disabled_next}>{ ">" }</button>

      <p class="loading">
        {
          if state.loading {
            "Loading url..."
          } else if !*image_loaded {
            "Loading image..."
          } else {
            "Finished."
          }
        }
      </p>

      <p class="error">
        { if let Some(error) = &state.error { error } else { "" } }
      </p>

      <p>
        {
          if let Some(url) = &state.data {
            html! {
              <a class="image" href={ url.to_owned() } target="_blank">
                <img src={ url.to_owned() } onload={onload_image} />
              </a>
            }
          } else {
            html! { }
          }
        }
      </p>

      /*TODO Previous comics */
    </>
  }
}

fn main() {
  yew::start_app::<App>();
}
