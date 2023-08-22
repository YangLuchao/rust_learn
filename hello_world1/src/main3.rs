// 生命周期标注后的代码
struct Foo;

impl Foo {
    // 生命周期消除规则第三条:
    // 若存在多个输入生命周期，且其中一个是 `&self` 或 `&mut self`，则 `&self` 的生命周期被赋给所有的输出生命周期
    fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
        // mutate_and_share 方法中，参数 &mut self 和返回值 &self 的生命周期是相同的
        // 因此，若返回值的生命周期在 main 函数有效，那 &mut self 的借用也是在 main 函数有效。
        &'a *self
    }
    fn share<'a>(&'a self) {}
}
// 总结下：&mut self 借用的生命周期和 loan 的生命周期相同，将持续到 println 结束。
// 而在此期间 foo.share() 又进行了一次不可变 &foo 借用，违背了可变借用与不可变借用不能同时存在的规则，
// 最终导致了编译错误。
fn main() {
    'b: {
        let mut foo: Foo = Foo;
        'c: {
            // 可以注意到 &mut foo 和 loan 的生命周期都是 'c
            let loan: &'c Foo = Foo::mutate_and_share::<'c>(&'c mut foo);
            'd: {
                Foo::share::<'d>(&'d foo);
            }
            println!("{:?}", loan);
        }
    }
}