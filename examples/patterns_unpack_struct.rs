#![feature(exclusive_range_pattern)]
/*!
# 解释下过程宏中比较难懂的下面这段代码
```no_run
let fields = if let syn::Data::Struct(syn::DataStruct {
    fields: syn::Fields::Named(ref fields),
    ..
}) = ast.data {
    fields
};
```

这段代码是提取named结构体里的字段，有点绕/难理解，因为用到两个Rust结构体表达式
1. struct unpack: 类似Python的*list
2. struct remaining fields(struct's .. syntax)

## Example

```
struct A {
    a: i32,
    b: i32,
}

let a1 = A { a: 1, b: 2 };
// unpack struct aa
let A { a: a_rename, b } = a1;
// struct remaining fields b construct from a1.b
let a2 = A { a: 1, ..a1 };
let a3 = if let A { a, ..} = a2{};
dbg!(a3);
assert_eq!(a_rename, 1);
assert_eq!(b, 2);
```

再详细解释下面这段代码是如何逐层unpack ast.data, ast=abstract syntax tree
1. Derive只能以类似Python装饰器的方式加到struct/enum/union上，
   所以第一层`syn::Data::Struct()`将ast.data约束成struct
2. struct又能分为Named、Unnamed(类似typle)、Unit(空结构体)三种
   第一层的括号内的syn::DataStruct {}是一种struct unpack写法，这里简称为第二层
   这里第二层的syn::Fields::Named再次把struct限制为Named struct一种

===

在protobuf中`..`remain wildcard pattern(另一个wildcard pattern是`_`表示match any single val)有填充默认值的作用
```no_run
let sign_up_req_pb_msg = PBMessage {
    api_type: PBAPIType::SignUp,
    request: ClientRequest::sign_up_request(
        email: user.email.clone(),
        password: user.password.clone(),
        ..Default::default()
    ),
    ..Default::default()
};
```
*/

/**
patterns matching这部分应该多看[reference的文档](https://doc.rust-lang.org/reference/patterns.html)
不要看by example或the book，只有reference才是最全的
feature: exclusive_range_pattern
*/
fn is_adult(age: u8) -> bool {
    match age {
        _age_u8 @ 0..18 => false,
        60..100 => {
            println!("older than 60");
            true
        },
        _ => true
    }
}

use std::collections::HashSet;
struct WsChannel {
    subscribe_list: Option<HashSet<String>>
}

/**
为什么需要ref关键字(pattern)?
因为从编译器parser的角度，`if let Some(ref var_name) = self.list`中
var_name是一个变量名(ident)，只有&Type这样引用符号+类型的写法，let绑定中没有let &var_name的写法
但是又想实现&type_name的效果，于是引入了let ref var_name
在let Some()的括号内只能写变量名信息，不能写类型，所以也不能想当然的指定下类型 let Some(var_name: &type_name)
*/
fn test_struct_ws_channel() {
    let params = WsChannel { subscribe_list: Some(HashSet::new()) };
    let params_ref = &params;
    // 稍微复杂点的struct ref，对某个option成员进行unwrap，Rust的自动加ref和自动解引用就不好使了，需要手动使用ref pattern
    // If a binding pattern does not explicitly have ref, ref mut, or mut, then it uses the default binding mode to determine how the variable is bound. The default binding mode starts in "move" mode which uses move semantics
    if let Some(ref fields) = params_ref.subscribe_list {
        dbg!(fields.len());
    }
}

fn main() {
    assert!(!is_adult(17));
    assert!(is_adult(66));
    test_struct_ws_channel();
}