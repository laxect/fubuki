/// article list component
use crate::{
    loading::Loading,
    style::Colors,
    utils::{use_json, use_title},
    Route,
};
pub use fubuki_types::{Post, PostList};
use stylist::yew::{styled_component, use_style};
use yew::{classes, html, use_context, use_state_eq, Callback, Html, Properties};
use yew_router::components::Link;

#[derive(Clone, PartialEq)]
pub enum PageNumMod {
    Prev,
    Next,
}

impl PageNumMod {
    pub fn value(&self) -> &'static str {
        match self {
            PageNumMod::Prev => "← Prev",
            PageNumMod::Next => "Next →",
        }
    }
}

#[inline]
fn page_count(postlist: &PostList) -> usize {
    (postlist.len() + 4) / 5
}

fn page_flip(page_num: usize, msg: &PageNumMod, page_count: usize) -> usize {
    let mut new_page_num = match msg {
        PageNumMod::Prev => page_num.saturating_sub(1),
        PageNumMod::Next => page_num.saturating_add(1),
    };
    if new_page_num > page_count {
        new_page_num = page_count;
    }
    new_page_num
}

#[derive(Clone, PartialEq, Properties)]
struct ArticleItemProps {
    post: Post,
}

#[styled_component(ArticleItem)]
fn article_item(props: &ArticleItemProps) -> Html {
    let ArticleItemProps { post } = props;
    let route = Route::Post { id: post.url.clone() };
    let colors: Colors = use_context().unwrap();

    // styles
    let split_line = use_style!(
        "padding-bottom: 2em; border-bottom: solid 0.05em ${underground};",
        underground = colors.underground
    );
    let post_title = use_style!(
        r#"
    padding: 0;
    transform: perspective(1px) translateZ(0);
    color: ${bold};
    &:visited {
        color: ${bold};
    }
    &::before {
        content: "";
        position: absolute;
        z-index: -1;
        left: 0;
        right: 0;
        bottom: -0.3rem;
        width: 0.7rem;
        background: ${bold};
        height: 0.15em;
        transition-property: width;
        transition-duration: 0.6s;
        transition-timing-function: ease-out;
    }
    &:hover::before {
        width: 4rem;
    }"#,
        bold = colors.bold
    );
    let time = use_style!(
        "display: inline-block; width: 9em; color: ${shadow};",
        shadow = colors.shadow
    );
    let category = use_style!(
        "margin-left: 0; padding: 0.3em 0.5em; color: ${fg}; background-color: ${bg};",
        fg = colors.rev_fg,
        bg = colors.rev_bg
    );
    let spoiler = use_style!(
        "padding: 0.3em 0.5em; color: ${fg}; background-color: ${bg};",
        fg = colors.colors_fg,
        bg = colors.red_bg
    );

    html! {
        <article class={split_line}>
            <h2>
                <Link<Route> to={route} classes={classes![post_title]}>{&post.title}</Link<Route>>
            </h2>
            <p>{ &post.summary }</p>
            <small>
                <time class={time}>{ &post.date }</time>
                <span class={category}>{ &post.category }</span>
                {
                    if post.has_spoiler() {
                        html! {
                            <span class={spoiler}>{ "ネタバレ注意" }</span>
                        }
                    } else {
                        html! {}
                    }
                }
            </small>
        </article>
    }
}

#[styled_component(Posts)]
pub fn posts() -> Html {
    let postlist: Vec<Post> = use_json(Route::posts_url()).unwrap_or_default();
    let page_num = use_state_eq(|| 0);

    let colors: Colors = use_context().unwrap();
    use_title("ポスト");
    gloo_utils::document().document_element().unwrap().set_scroll_top(0);

    // nav button
    let page_count = page_count(&postlist);
    let style_nav_button = use_style!(
        "
      padding: 0.3em 0.2em;
      border-top: solid ${underground} 0.2em;
      &:hover {
        border-top: solid ${normal} 0.2em;
        background-color: ${bg2};
      }
      transition-property: all;
      transition-duration: 0.3s;
      transition-timing-function: ease-out;
",
        underground = colors.underground,
        normal = colors.normal,
        bg2 = colors.bg2,
    );

    let style_disable = use_style!("pointer-events: none; color: ${shadow};", shadow = colors.shadow);
    let link = |item: PageNumMod, page_num: yew::UseStateHandle<usize>| -> Html {
        let disabled = match item {
            PageNumMod::Next => *page_num + 1 >= page_count,
            PageNumMod::Prev => *page_num == 0,
        };
        let mark = item.value();

        let mut class = classes![style_nav_button.clone()];
        if disabled {
            class.push(style_disable.clone());
        }
        let next = page_flip(*page_num, &item, page_count);
        let onclick = Callback::from(move |_| page_num.set(next));
        html! {
            <button {class} {onclick}>
                { mark }
            </button>
        }
    };

    // List
    let style_nav = use_style!(
        "
            margin-top: 1em;
            margin-left: -0.5em;
            display: grid;
            grid-template-columns: repeat(2, 4em);
            grid-gap: 1.5em;"
    );
    if postlist.is_empty() {
        html! {
             <Loading />
        }
    } else {
        let start = *page_num * 5;
        let postlist = postlist
            .iter()
            .skip(start)
            .take(5)
            .cloned()
            .map(|post| {
                let key = post.url.clone();
                html! {<ArticleItem {key} {post}/>}
            })
            .collect::<Html>();
        html! {
            <>
            { postlist }
               <nav class={style_nav} style="float: right">
                    { link(PageNumMod::Prev, page_num.clone()) }
                    { link(PageNumMod::Next, page_num.clone()) }
                </nav>
            </>
        }
    }
}
