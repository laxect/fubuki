use stylist::yew::{styled_component, use_style};
use yew::{classes, html, use_context};

use crate::style::Colors;

#[styled_component(Loading)]
pub fn loading() -> Html {
    let colors: Colors = use_context().unwrap();
    let style = use_style!(
        "
width: 500px;
height: 500px;
position: relative;
left: 50%;
transform: translateX(-50%);

.outer-circle, .inner-circle {
    position: absolute;
    border-radius: 50%;
    border: 2px solid ${final};
    top: 50%;
    left: 50%;
    transform: translateX(-50%) translateY(-50%);
    animation: ani 1s 1;
}
.outer-circle {
    width: 320px;
    height: 320px;
}
.inner-circle {
    width: 285px;
    height: 285px;
}
.z1 {
    width: 200px;
    height: 200px;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translateX(-50%) translateY(-50%);
}
.block1, .block2 {
    width: 200px;
    height: 200px;
    position: absolute;
    border: 2px solid ${final};
    top: 50%;
    left: 50%;
    animation: ani 1s 1;
}
.block1 {
    transform: translateX(-50%) translateY(-50%) rotate(0deg);
}
.block2 {
    transform-origin: center;
    transform: translateX(-50%) translateY(-50%) rotate(45deg);
}
@keyframes ani {
    0%{
        border-color: ${start};
    }
    100%{
        border-color: ${final};
    }
}
", start = colors.background, final = colors.bold
    );
    html! {
    <div class={classes!["wai", style]}>
        <div class="outer-circle"></div>
        <div class="inner-circle"></div>
        <div class="z1">
            <div class="block1"></div>
            <div class="block2"></div>
        </div>
    </div>
    }
}
