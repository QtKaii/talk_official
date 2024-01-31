use maud::{html, Markup, DOCTYPE};

pub async fn index() -> Markup {
  html! {
      (DOCTYPE)
      html {
          head {
              title { "Hello, World!" }
              script src="/javascript/htmx" {}
          }
          body {

          }
      }
  }
}