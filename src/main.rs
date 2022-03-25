mod app;
mod layout;
mod pages;
mod sidemenu;
mod utils;

// 使用 wee_alloc 替代默认的内存分配器，以牺牲一定速度与内存为代价获得更小的尺寸
// https://yew.rs/docs/advanced-topics/optimizations#wee_alloc
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    use self::app::App;

    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
