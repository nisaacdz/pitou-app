use yew::prelude::*;

mod props;

pub use props::*;
mod clicks;

pub use clicks::*;

#[function_component]
pub fn DirIcon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="70%" height="70%" viewBox="0 0 48 48">
            <path fill="#FFA000" d="M38,12H22l-4-4H8c-2.2,0-4,1.8-4,4v24c0,2.2,1.8,4,4,4h31c1.7,0,3-1.3,3-3V16C42,13.8,40.2,12,38,12z"></path>
            <path fill="#FFCA28" d="M42.2,18H15.3c-1.9,0-3.6,1.4-3.9,3.3L8,40h31.7c1.9,0,3.6-1.4,3.9-3.3l2.5-14C46.6,20.3,44.7,18,42.2,18z"></path>
        </svg>
    }
}

#[function_component]
pub fn StackedDirIcon() -> Html {
    html! {
        <img src="./public/icons/folder_with_content_icon.svg" alt="Stacked Folder" />
    }
}

#[function_component]
pub fn TxtIcon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="60%" height="60%" viewBox="0 0 172 172" style=" fill:#26e07f;">
            <g fill="none" fill-rule="nonzero" stroke="none" stroke-width="1" stroke-linecap="butt" stroke-linejoin="miter" stroke-miterlimit="10" stroke-dasharray="" stroke-dashoffset="0" font-family="none" font-weight="none" font-size="none" text-anchor="none" style="mix-blend-mode: normal">
                <path d="M0,172v-172h172v172z" fill="none"></path>
                <g fill="#1fb141">
                    <path d="M21.5,21.5v129h64.5v-32.25v-64.5v-32.25zM86,53.75c0,17.7805 14.4695,32.25 32.25,32.25c17.7805,0 32.25,-14.4695 32.25,-32.25c0,-17.7805 -14.4695,-32.25 -32.25,-32.25c-17.7805,0 -32.25,14.4695 -32.25,32.25zM118.25,86c-17.7805,0 -32.25,14.4695 -32.25,32.25c0,17.7805 14.4695,32.25 32.25,32.25c17.7805,0 32.25,-14.4695 32.25,-32.25c0,-17.7805 -14.4695,-32.25 -32.25,-32.25z"></path>
                </g>
            </g>
        </svg>
    }
}

#[function_component]
pub fn PropertiesIcon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="70%" height="70%" viewBox="0 0 172 172">
            <g fill="none" fill-rule="nonzero" stroke="none" stroke-width="1" stroke-linecap="butt" stroke-linejoin="miter" stroke-miterlimit="10" stroke-dasharray="" stroke-dashoffset="0" font-family="none" font-weight="none" font-size="none" text-anchor="none" style="mix-blend-mode: normal">
                <path d="M0,172v-172h172v172z" fill="none"></path>
                <g fill="#1fb141">
                    <path d="M21.5,21.5v129h64.5v-32.25v-64.5v-32.25zM86,53.75c0,17.7805 14.4695,32.25 32.25,32.25c17.7805,0 32.25,-14.4695 32.25,-32.25c0,-17.7805 -14.4695,-32.25 -32.25,-32.25c-17.7805,0 -32.25,14.4695 -32.25,32.25zM118.25,86c-17.7805,0 -32.25,14.4695 -32.25,32.25c0,17.7805 14.4695,32.25 32.25,32.25c17.7805,0 32.25,-14.4695 32.25,-32.25c0,-17.7805 -14.4695,-32.25 -32.25,-32.25z"></path>
                </g>
            </g>
        </svg>
    }
}

#[function_component]
pub fn RefreshIcon(prop: &ThemeProp) -> Html {
    let basic_color = prop.theme().background2();
    let style = format! {"stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: {basic_color}; fill-rule: nonzero; opacity: 1;"};

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 256 256">
            <g style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)" >
                <path d="M 56.375 60.616 c -0.75 -1.027 -2.151 -1.388 -3.266 -0.776 c -2.472 1.356 -5.237 2.065 -8.109 2.065 c -7.2 0 -13.346 -4.532 -15.778 -10.887 h 5.968 c 0.734 0 1.41 -0.402 1.759 -1.049 c 0.349 -0.646 0.316 -1.432 -0.085 -2.046 L 20.269 22.551 c -0.37 -0.565 -0.999 -0.905 -1.674 -0.905 s -1.304 0.34 -1.674 0.905 L 0.326 47.924 c -0.402 0.614 -0.435 1.4 -0.085 2.046 C 0.59 50.616 1.266 51.019 2 51.019 h 7.609 C 12.482 67.96 27.253 80.905 45 80.905 c 6.736 0 13.203 -1.841 18.864 -5.348 c 1.272 -0.788 1.586 -2.509 0.704 -3.718 L 56.375 60.616 z" style = { style.clone() } transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
                <path d="M 89.759 39.227 c -0.349 -0.646 -1.024 -1.048 -1.759 -1.048 h -7.753 C 77.051 21.632 62.465 9.095 45 9.095 c -6.086 0 -12.01 1.525 -17.291 4.432 c -1.307 0.719 -1.7 2.42 -0.883 3.668 l 7.613 11.628 c 0.694 1.06 2.072 1.499 3.213 0.947 c 2.277 -1.1 4.78 -1.675 7.348 -1.675 c 6.893 0 12.827 4.153 15.455 10.083 h -5.645 c -0.734 0 -1.41 0.402 -1.759 1.048 c -0.35 0.646 -0.317 1.432 0.085 2.046 l 16.595 25.373 c 0.369 0.564 0.999 0.905 1.674 0.905 s 1.305 -0.341 1.674 -0.905 l 16.595 -25.373 C 90.076 40.658 90.108 39.873 89.759 39.227 z" {style} transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
            </g>
        </svg>
    }
}

#[function_component]
pub fn SettingsIcon(prop: &ThemeProp) -> Html {
    let color = prop.theme().background2();

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="80%" height="80%" viewBox="0 0 48 48">
            <path fill={ format!("{}", color) } d="M39.6,27.2c0.1-0.7,0.2-1.4,0.2-2.2s-0.1-1.5-0.2-2.2l4.5-3.2c0.4-0.3,0.6-0.9,0.3-1.4L40,10.8c-0.3-0.5-0.8-0.7-1.3-0.4l-5,2.3c-1.2-0.9-2.4-1.6-3.8-2.2l-0.5-5.5c-0.1-0.5-0.5-0.9-1-0.9h-8.6c-0.5,0-1,0.4-1,0.9l-0.5,5.5c-1.4,0.6-2.7,1.3-3.8,2.2l-5-2.3c-0.5-0.2-1.1,0-1.3,0.4l-4.3,7.4c-0.3,0.5-0.1,1.1,0.3,1.4l4.5,3.2c-0.1,0.7-0.2,1.4-0.2,2.2s0.1,1.5,0.2,2.2L4,30.4c-0.4,0.3-0.6,0.9-0.3,1.4L8,39.2c0.3,0.5,0.8,0.7,1.3,0.4l5-2.3c1.2,0.9,2.4,1.6,3.8,2.2l0.5,5.5c0.1,0.5,0.5,0.9,1,0.9h8.6c0.5,0,1-0.4,1-0.9l0.5-5.5c1.4-0.6,2.7-1.3,3.8-2.2l5,2.3c0.5,0.2,1.1,0,1.3-0.4l4.3-7.4c0.3-0.5,0.1-1.1-0.3-1.4L39.6,27.2z M24,35c-5.5,0-10-4.5-10-10c0-5.5,4.5-10,10-10c5.5,0,10,4.5,10,10C34,30.5,29.5,35,24,35z"></path>
            <path fill={ format!("{}", color) } d="M24,13c-6.6,0-12,5.4-12,12c0,6.6,5.4,12,12,12s12-5.4,12-12C36,18.4,30.6,13,24,13z M24,30c-2.8,0-5-2.2-5-5c0-2.8,2.2-5,5-5s5,2.2,5,5C29,27.8,26.8,30,24,30z"></path>
        </svg>
    }
}

#[function_component]
pub fn HomeIcon(prop: &ThemeProp) -> Html {
    let color = prop.theme().background2();

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="70%" height="70%" viewBox="0 0 256 256">
            <g style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)">
                <polygon points="75.96,30.96 75.96,13.34 67.26,13.34 67.26,22.26 45,0 0.99,44.02 7.13,50.15 45,12.28 82.88,50.15 89.01,44.02" style={ format!("fill: {};", color) } transform="matrix(1 0 0 1 0 0)" />
                <polygon points="45,20 14.04,50.95 14.04,90 35.29,90 35.29,63.14 54.71,63.14 54.71,90 75.96,90 75.96,50.95" style={ format!("fill: {};", color) } transform="matrix(1 0 0 1 0 0)" />
            </g>
        </svg>
    }
}

#[function_component]
pub fn InfoIcon(prop: &ThemeProp) -> Html {
    let color = prop.theme().background2();

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="67%" height="67%" viewBox="0 0 256 256">
            <g style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)">
                <path d="M 45 0 C 20.147 0 0 20.147 0 45 c 0 24.853 20.147 45 45 45 s 45 -20.147 45 -45 C 90 20.147 69.853 0 45 0 z M 50.184 16.439 c 3.615 0 6.546 2.931 6.546 6.546 c 0 3.615 -2.931 6.546 -6.546 6.546 c -3.615 0 -6.546 -2.931 -6.546 -6.546 C 43.639 19.37 46.569 16.439 50.184 16.439 z M 36.792 64.541 l 4.867 -18.158 l 0.955 -3.585 c 2.128 -7.024 -8.547 -4.482 -11.366 -2.498 L 31 37.464 c 3.096 -2.416 24.252 -10.446 22.208 -0.991 l -2.446 8.91 l -3.375 12.833 c -2.128 7.024 8.547 4.482 11.365 2.498 L 59 63.551 C 55.904 65.967 34.748 73.997 36.792 64.541 z" style={ format!("fill: {};", color) } transform="matrix(1 0 0 1 0 0)" stroke-linecap="round" />
            </g>
        </svg>
    }
}

#[function_component]
pub fn LockedIcon2() -> Html {
    html! {
        <img src="./public/icons/locked_icon_2.svg" alt="Locked Files" />
    }
}

#[function_component]
pub fn BackIcon(prop: &ThemeProp) -> Html {
    let color = prop.theme().background2();

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="70%" height="70%" viewBox="0 0 256 256">
            <g style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)">
                <path d="M 42.362 14.245 L 42.362 14.245 c 4.165 4.15 4.171 10.892 0.014 15.05 l -5.077 5.077 l 41.069 0 C 84.24 34.371 89 39.131 89 45.003 v 0 v 0 c 0 5.872 -4.76 10.632 -10.632 10.632 H 37.299 l 5.063 5.063 c 4.152 4.152 4.152 10.884 0 15.036 l 0 0 c -4.152 4.152 -10.884 4.152 -15.036 0 L 4.114 52.521 c -4.152 -4.152 -4.152 -10.884 0 -15.036 L 27.34 14.259 C 31.487 10.112 38.208 10.106 42.362 14.245 z" style={ format!("fill: {};", color) } transform="matrix(1 0 0 1 0 0)" stroke-linecap="round" />
                <path d="M 34.844 79.847 c 3.107 0 6.028 -1.21 8.226 -3.407 c 2.197 -2.196 3.407 -5.117 3.407 -8.225 c 0 -3.106 -1.21 -6.027 -3.407 -8.225 l -3.356 -3.355 h 38.655 C 84.782 56.635 90 51.417 90 45.003 c 0 -6.414 -5.218 -11.632 -11.632 -11.632 H 39.713 l 3.37 -3.37 c 2.2 -2.2 3.41 -5.124 3.407 -8.235 c -0.002 -3.111 -1.218 -6.034 -3.421 -8.23 l 0 0 c -4.534 -4.517 -11.906 -4.512 -16.435 0.016 L 3.407 36.778 C 1.21 38.975 0 41.896 0 45.002 s 1.21 6.028 3.407 8.225 l 23.212 23.212 C 28.816 78.637 31.737 79.847 34.844 79.847 z M 78.368 35.371 c 5.311 0 9.632 4.321 9.632 9.631 c 0 5.311 -4.321 9.632 -9.632 9.632 H 37.299 c -0.404 0 -0.77 0.244 -0.924 0.617 c -0.155 0.374 -0.069 0.804 0.217 1.09 l 5.063 5.063 c 1.819 1.819 2.821 4.238 2.821 6.811 c 0 2.573 -1.001 4.991 -2.821 6.811 c -3.755 3.755 -9.865 3.757 -13.622 0 L 4.821 51.813 C 3.002 49.994 2 47.575 2 45.002 s 1.002 -4.991 2.821 -6.811 l 23.227 -23.226 c 3.75 -3.749 9.854 -3.754 13.608 -0.013 c 1.825 1.818 2.831 4.238 2.833 6.815 c 0.002 2.576 -1 4.998 -2.821 6.819 l -5.077 5.077 c -0.286 0.286 -0.372 0.716 -0.217 1.09 c 0.154 0.374 0.52 0.617 0.924 0.617 H 78.368 z" style= { format!("fill: {};", color) } transform="matrix(1 0 0 1 0 0)" stroke-linecap="round" />
            </g>
        </svg>
    }
}

#[function_component]
pub fn SearchIcon(prop: &ThemeProp) -> Html {
    let basic_color = prop.theme().background2();
    let style = format! {"stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: {basic_color}; fill-rule: nonzero; opacity: 1;"};

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" width="70%" height="70%" viewBox="0 0 256 256">
            <g style="stroke: none; stroke-width: 0; stroke-dasharray: none; stroke-linecap: butt; stroke-linejoin: miter; stroke-miterlimit: 10; fill: none; fill-rule: nonzero; opacity: 1;" transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)" >
                <path d="M 87.803 77.194 L 68.212 57.602 c 9.5 -14.422 7.912 -34.054 -4.766 -46.732 c 0 0 -0.001 0 -0.001 0 c -14.495 -14.493 -38.08 -14.494 -52.574 0 c -14.494 14.495 -14.494 38.079 0 52.575 c 7.248 7.247 16.767 10.87 26.287 10.87 c 7.134 0 14.267 -2.035 20.445 -6.104 l 19.591 19.591 C 78.659 89.267 80.579 90 82.498 90 s 3.84 -0.733 5.305 -2.197 C 90.732 84.873 90.732 80.124 87.803 77.194 z M 21.48 52.837 c -8.645 -8.646 -8.645 -22.713 0 -31.358 c 4.323 -4.322 10 -6.483 15.679 -6.483 c 5.678 0 11.356 2.161 15.678 6.483 c 8.644 8.644 8.645 22.707 0.005 31.352 c -0.002 0.002 -0.004 0.003 -0.005 0.005 c -0.002 0.002 -0.003 0.003 -0.004 0.005 C 44.184 61.481 30.123 61.48 21.48 52.837 z" {style} transform=" matrix(1 0 0 1 0 0) " stroke-linecap="round" />
            </g>
        </svg>
    }
}

#[function_component]
pub fn LockedIcon(_prop: &ThemeProp) -> Html {
    html! {
        <img src="./public/icons/locked_icon.svg" alt="locked" />
    }
}

#[function_component]
pub fn CloudIcon(_prop: &ThemeProp) -> Html {
    html! {
        <svg baseProfile="tiny" height="24px" id="Layer_1" version="1.2" viewBox="0 0 24 24" width="24px" xmlns="http://www.w3.org/2000/svg">
        <path d="M17,9c-0.115,0-0.231,0.005-0.351,0.015C15.824,6.638,13.587,5,11,5c-3.309,0-6,2.691-6,6c0,0.042,0,0.084,0.001,0.126  C3.277,11.571,2,13.139,2,15c0,2.206,1.794,4,4,4h5v-4.586l-1.293,1.293C9.512,15.902,9.256,16,9,16s-0.512-0.098-0.707-0.293  c-0.391-0.391-0.391-1.023,0-1.414l2.999-2.999c0.093-0.093,0.203-0.166,0.326-0.217c0.244-0.101,0.52-0.101,0.764,0  c0.123,0.051,0.233,0.124,0.326,0.217l2.999,2.999c0.391,0.391,0.391,1.023,0,1.414C15.512,15.902,15.256,16,15,16  s-0.512-0.098-0.707-0.293L13,14.414V19h4c2.757,0,5-2.243,5-5S19.757,9,17,9z"/>
        </svg>
    }
}

#[function_component]
pub fn CloudStorageIcon(_prop: &ThemeProp) -> Html {
    html! {
        <svg id="Layer_1_1_" style="fill: #7ACED7; enable-background:new 0 0 16 16;" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
        <path d="M13.778,7.556c-0.002,0-0.005,0.001-0.007,0.001c0.171-0.806,0.137-1.686-0.296-2.622C12.862,3.61,11.607,2.62,10.16,2.436  C8.148,2.18,6.369,3.327,5.66,5.032C5.421,4.94,5.161,4.889,4.889,4.889c-1.227,0-2.222,0.995-2.222,2.222  c0,0.176,0.025,0.344,0.064,0.508C2.198,7.494,1.614,7.508,0.906,8.08C0.38,8.506-0.007,9.126,0,9.803  C0.014,11.019,1.003,12,2.222,12h11.449c1.039,0,2.019-0.661,2.262-1.671C16.286,8.863,15.185,7.556,13.778,7.556z"/>
        </svg>
    }
}

#[function_component]
pub fn PasteIcon(_prop: &ThemeProp) -> Html {
    html! {
        <img src="./public/icons/paste_icon.svg" alt="Paste Icon" />
    }
}

#[function_component]
pub fn CutIcon(_prop: &ThemeProp) -> Html {
    html! {
        <img src="./public/icons/paste_icon.svg" alt="cut" />
    }
}

#[function_component]
pub fn CopyIcon(_prop: &ThemeProp) -> Html {
    //TODO
    html! {
        <img src="./public/icons/paste_icon.svg" alt="copy" />
    }
}

#[function_component]
pub fn BookmarksIcon() -> Html {
    html! {
        <img src="./public/icons/favorites_icon.svg" alt="bookmarks" />
    }
}

#[function_component]
pub fn HistoryIcon(_prop: &ThemeProp) -> Html {
    html! {
        <img src="./public/icons/history_icon.svg" alt="history" />
    }
}
