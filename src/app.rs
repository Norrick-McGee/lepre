use leptos::*;
use leptos_meta::*;
//use leptos_router::*;

use crate::model::conversation::Conversation;
use crate::model::conversation::Message;
use crate::api::converse;


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    
    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };

        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });
        converse(cx, conversation.get())

    });
    
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
    
        // sets the document title
        <Title text="Lepre - your lil pot of gold"/>
        // <ChatArea conversation/>
        // <TypeArea send/>
    }
}

