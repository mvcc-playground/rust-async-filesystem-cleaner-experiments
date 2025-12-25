use axum::{Router, routing::get};
use maud::{DOCTYPE, Markup, PreEscaped, html};
use ulid::Ulid;

/// A basic page layout with dynamic title and content.
fn layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { (title) }
                // Link solicitado explicitamente
                // link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/tailwindcss@4.1.18/index.css";
                // Importante: O runtime do Tailwind v4 deve ser carregado como um script, não stylesheet.
                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}

                // Opcional: HTMX
                script src="https://cdn.jsdelivr.net/npm/htmx.org@1.9.10" {}

                // (Script v3 removido para evitar conflito com v4)
            }
            body {
                (content)
            }
            (PreEscaped(
                r#"<style type="text/tailwindcss">
                    #container {
                        @apply flex flex-col gap-4 items-center justify-center h-screen;
                    }
                    #btn {
                        @apply bg-zinc-500 text-white rounded-md px-4 py-2;
                    } 
                </style>"#
            ))
            // (PreEscaped("<style type=\"text/tailwindcss\"> button { @apply bg-red-500 text-white rounded-md px-4 py-2; } </style>"))

        }
    }
}

async fn hello_world() -> Markup {
    let id = format!("id-{}", Ulid::new().to_string());
    let hp_class = format!("h1-{}-class", Ulid::new().to_string());

    layout(
        "Maud + HTMX + Tailwind",
        html! {
            div id="container" {
                h1 class="text-2xl text-black" { "Hello, World!" }
                p class=(hp_class) { "This is a paragraph." }
                button id=(id) class="bg-red-500 text-white rounded-md px-3 py-1" onclick="alert('Hello, World!')" { "Overridden Button" }
                button id="btn" onclick="alert('Hello, World!')" { "Styled by @apply" }
            }

            // Exemplo de CSS inline (raw) se necessário, mas com Tailwind é menos usado
            (PreEscaped(format!("<style> .{} {{ color: red; }} </style>", hp_class)))
            (PreEscaped(format!("<style> #{} {{ border-radius: 0.75rem; padding: 0.5rem 0.75rem; }} </style>", id)))
            // Para usar @apply no navegador com o script do Tailwind, é necessário type="text/tailwindcss"
        },
    )
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(hello_world));

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
