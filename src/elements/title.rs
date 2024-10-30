use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// Maintain the normal spacing between titles and subtitles.
    #[prop_or_default]
    pub is_spaced: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let class = classes!(
        "title",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.is_spaced.then_some("is-spaced"),
    );
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SubtitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
#[function_component(Subtitle)]
pub fn subtitle(props: &SubtitleProps) -> Html {
    let class = classes!("subtitle", props.classes.clone(), props.size.as_ref().map(|size| size.to_string()));
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

/// The six sizes available for titles & subtitles.
///
/// https://bulma.io/documentation/elements/title/#sizes
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("is-{_variant}")]
pub enum HeaderSize {
    #[display("1")]
    Is1,
    #[display("2")]
    Is2,
    #[display("3")]
    Is3,
    #[display("4")]
    Is4,
    #[display("5")]
    Is5,
    #[display("6")]
    Is6,
}
