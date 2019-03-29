import blog from './blog/Cargo.toml'
import han_css from './node_modules/han-css/dist/han.css'
import main from './main.sass'
import han from './node_modules/han-css/dist/han'

window.han = han.init();
window.addEventListener('loaded', (_e) => {
    window.han.render();
});
