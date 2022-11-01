use chrono::{Date, Duration, Utc};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_is_first_mount};

//TODO Move all components to comps.rs module

/// Kind of action to set new date
enum SetDate {
  Today,
  Random,
  Prev,
  Next,
  Custom(Event),
}

/// Whole app (OOP)
struct App {
  date: Date<Utc>,
}

impl Component for App {
  type Message = SetDate;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    Self {
      date: garf::today_date(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      SetDate::Today => {
        self.date = garf::today_date();
        true
      }
      SetDate::Random => {
        self.date = garf::random_date();
        true
      }
      SetDate::Prev => {
        self.date -= Duration::days(1);
        true
      }
      SetDate::Next => {
        self.date += Duration::days(1);
        true
      }
      SetDate::Custom(e) => {
        let target: HtmlInputElement = e.target_unchecked_into();
        self.date =
          garf::input_string_to_date(&target.value()).expect("Input date not properly formatted");
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link();
    let date_str = garf::date_to_string(self.date, "-", true);

    html! {
      <>
        <h1>{ "Garf" }</h1>

        // Change date actions
        <div class="date">
          <input type="date" value={ date_str } onchange={link.callback(|e| SetDate::Custom(e) )} />

          <button onclick={link.callback(|_| SetDate::Today)}>{  "Today"  }</button>
          <button onclick={link.callback(|_| SetDate::Random)}>{ "Random" }</button>
          <button onclick={link.callback(|_| SetDate::Prev)}>{   "Prev"   }</button>
          <button onclick={link.callback(|_| SetDate::Next)}>{   "Next"   }</button>
        </div>

        // Comic component
        <Comic date={ self.date } />
      </>
    }
  }
}

/// Properties parameter for `Comic` component
#[derive(Properties, PartialEq)]
struct ComicProps {
  date: Date<Utc>,
}

/// Must be functional for `use_async` hook
#[function_component(Comic)]
fn comic(props: &ComicProps) -> Html {
  log::debug!("Run comic {}", props.date);

  // Create async fetch hook
  let state = use_async(garf::get_comic_url(props.date));

  // Run if initial component mount (page load)
  if use_is_first_mount() {
    log::debug!("First run state {}", props.date);
    state.run();
  }

  // Create cache for date
  let date_cache = use_state(|| props.date);

  // Run if date is different that cache (last load)
  if *date_cache != props.date {
    log::debug!("Other run state {}", props.date);
    date_cache.set(props.date);
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

  html! {
    <div class="comic">

      // * Debug
      <p>{ garf::date_to_string(props.date, "/", true) }</p>

      // Load status
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

      // Error status
      <p class="error">
        { if let Some(error) = &state.error { error } else { "" } }
      </p>

      // Render if image url loaded
      {
        if let Some(url) = &state.data {
          // Link containing image
          html! {
            <a class="image" href={ url.to_owned() } target="_blank">
              <img src={ url.to_owned() } onload={ onload_image } />
            </a>
          }
        } else {
          html! { }
        }
      }
    </div>
  }
}

fn main() {
  yew::start_app::<App>();
}
