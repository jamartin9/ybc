use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use yew::prelude::*;

use yew_agent::worker::{use_worker_bridge, HandlerId, UseWorkerBridgeHandle, Worker, WorkerScope};

/// Modal actions.
pub enum ModalMsg {
    Open,
    Close,
    CloseFromAgent(ModalCloseMsg),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The content of the `"modal-content"` element.
    #[prop_or_default]
    pub children: Children,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Classes,
}

/// A classic modal overlay, in which you can include any content you want.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let is_active = use_state(|| false);

    let mut class = Classes::from("modal");

    class.push(props.classes.clone());

    let (opencb, closecb) = if *is_active {
        class.push("is-active");

        let is_active = is_active.clone();

        (Callback::noop(), Callback::from(move |_| is_active.set(false)))
    } else {
        let is_active = is_active.clone();

        (Callback::from(move |_| is_active.set(true)), Callback::noop())
    };

    {
        let id = props.id.clone();

        let _bridge: UseWorkerBridgeHandle<ModalCloser> = use_worker_bridge(move |response: ModalCloseMsg| {
            if response.0 == id {
                is_active.set(false);
            }
        });
    }

    html! {
        <>
        <div onclick={opencb}>
            {props.trigger.clone()}
        </div>
        <div id={props.id.clone()} {class}>
            <div class="modal-background" onclick={closecb.clone()}></div>
            <div class="modal-content">
                {props.children.clone()}
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
        </div>
        </>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalCardProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The title of this modal.
    pub title: String,
    /// The content to be placed in the `modal-card-body` not including the modal-card-header /
    /// modal-card-title, which is handled by the `modal_title` prop.
    #[prop_or_default]
    pub body: Html,
    /// The content to be placed in the `modal-card-footer`.
    #[prop_or_default]
    pub footer: Html,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Classes,
}

/// A classic modal with a header, body, and footer section.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(ModalCard)]
pub fn modal_card(props: &ModalCardProps) -> Html {
    let is_active = use_state(|| false);

    let mut class = Classes::from("modal");
    class.push(props.classes.clone());

    let (opencb, closecb) = if *is_active {
        class.push("is-active");

        let is_active = is_active.clone();

        (Callback::noop(), Callback::from(move |_| is_active.set(false)))
    } else {
        let is_active = is_active.clone();

        (Callback::from(move |_| is_active.set(true)), Callback::noop())
    };

    {
        let id = props.id.clone();

        let _bridge: UseWorkerBridgeHandle<ModalCloser> = use_worker_bridge(move |response: ModalCloseMsg| {
            if response.0 == id {
                is_active.set(false);
            }
        });
    }

    html! {
    <>
        <div onclick={opencb}>
            {props.trigger.clone()}
        </div>
        <div id={props.id.clone()} {class}>
            <div class="modal-background" onclick={closecb.clone()}></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{props.title.clone()}</p>
                    <button class="delete" aria-label="close" onclick={closecb.clone()}></button>
                </header>
                <section class="modal-card-body">
                    {props.body.clone()}
                </section>
                <footer class="modal-card-foot">
                    {props.footer.clone()}
                </footer>
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
        </div>
    </>
    }
}

//////////////////////////////////////////////////////////////////////////////

/// A request to close a modal instance by ID.
///
/// The ID provided in this message must match the ID of the modal which is to be closed, else
/// the message will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModalCloseMsg(pub String);

pub struct ModalCloser {
    subscribers: HashSet<HandlerId>,
    link: WorkerScope<Self>,
}

impl Worker for ModalCloser {
    type Input = ModalCloseMsg;
    type Message = ();
    // The agent receives requests to close modals by ID.
    type Output = ModalCloseMsg;

    // The agent forwards the input to all registered modals.

    fn create(link: &WorkerScope<Self>) -> Self {
        Self { subscribers: HashSet::new(), link: link.clone() }
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _: Self::Message) {}

    fn received(&mut self, _link: &WorkerScope<Self>, msg: Self::Input, _: HandlerId) {
        self.subscribers.iter().for_each(|cmp| {
            self.link.respond(*cmp, msg.clone());
        });
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
