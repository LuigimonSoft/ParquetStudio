use leptos::prelude::*;

/// Highlights basic SQL keywords by wrapping them in span tags with a Tailwind color.
pub fn highlight_sql(query: &str) -> String {
    fn escape_html(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    fn push_token(out: &mut String, tok: &str) {
        const KEYWORDS: [&str; 7] = [
            "SELECT", "FROM", "WHERE", "LIMIT", "INSERT", "UPDATE", "DELETE",
        ];
        let upper = tok.to_ascii_uppercase();
        if KEYWORDS.contains(&upper.as_str()) {
            out.push_str("<span class=\"text-emerald-400\">");
            out.push_str(&escape_html(tok));
            out.push_str("</span>");
        } else {
            out.push_str(&escape_html(tok));
        }
    }

    let mut result = String::new();
    let mut token = String::new();
    for ch in query.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            token.push(ch);
        } else {
            if !token.is_empty() {
                push_token(&mut result, &token);
                token.clear();
            }
            result.push_str(&escape_html(&ch.to_string()));
        }
    }
    if !token.is_empty() {
        push_token(&mut result, &token);
    }
    result
}

#[component]
pub fn QueryEditor(on_run: Callback<String, ()>) -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let run = move |_| on_run.run(query.get());
    let highlighted = Memo::new(move |_| highlight_sql(&query.get()));

    view! {
        <div class="flex flex-col bg-gray-800 p-4 rounded-md">
            <div class="relative h-40">
                <pre class="pointer-events-none absolute inset-0 overflow-auto p-2 font-mono text-sm whitespace-pre-wrap text-white">
                    <code inner_html=highlighted></code>
                </pre>
                <textarea
                    class="absolute inset-0 resize-none bg-transparent p-2 font-mono text-sm text-transparent caret-white"
                    on:input=move |ev| set_query.set(event_target_value(&ev))
                ></textarea>
            </div>
            <button class="mt-2 self-start px-3 py-1 bg-emerald-600 text-white rounded" on:click=run>"Run"</button>
        </div>
    }
}
