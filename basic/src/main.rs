mod print;
mod display;
mod control_flow;

fn main() {

    // 测试print
    print::run();

    // 测试display trait 手动和自动的实现
    display::run();

    // 测试Control flow
    control_flow::run();

}
