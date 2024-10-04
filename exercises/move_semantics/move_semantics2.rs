// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
/*在Rust中，数据默认是被移动的，而不是被复制的。这意味着当你把一个变量传递给一个函数时，这个变量的所有权就被转移到了那个函数，原来的变量就不能再使用了。这就是为什么在你的原始代码中，`vec0`在被传递给`fill_vec`函数后就不能再使用了。

然而，有些类型实现了`Clone` trait，这意味着它们可以被复制。当你调用`clone`方法时，你实际上是在创建一个原始数据的副本，这样你就可以保留原始数据的所有权，同时把它的一个副本传递给函数。

在你的代码中，`Vec<i32>`实现了`Clone` trait，所以你可以调用`vec0.clone()`来创建`vec0`的一个副本，并把这个副本传递给`fill_vec`函数。这样，`vec0`的所有权就保留在了`main`函数中，你可以在`fill_vec`调用后继续使用它。

总的来说，`clone`方法允许你在保留原始数据的同时，创建并使用它的副本。这就是为什么使用`clone`会使你的代码正常工作。希望这个解释对你有所帮助！*/