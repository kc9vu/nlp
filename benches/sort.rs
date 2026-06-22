use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn manual(mut v: Vec<String>) {
    v.sort_by(|a, b| nlp::natural_quick_cmp(a, b));
}

fn winnow(mut v: Vec<String>) {
    v.sort_by(|a, b| nlp::natural_cmp_filename(a, b));
}

fn bench_sort(c: &mut Criterion) {
    let v = vec![
        "0a37c1f535fc09b1416bee7f11404099285054730.jpg".to_string(),
        "2de7b55ee2433e371093bd2a096aa57f285054730.jpg".to_string(),
        "11fc754dc6165d47c13cb65ed1134ff7285054730.jpg".to_string(),
        "18e8ff277f881c7c0c1e3428ff6412f1285054730.jpg".to_string(),
        "48d55b437498a62b7fe77121372d1eac285054730.jpg".to_string(),
        "72df7b6fd84ac7618199403387478c40285054730.jpg".to_string(),
        "91a8e1a0efee825a958780d4c718f280285054730.jpg".to_string(),
        "98ce668f450b2476faac82671008157e285054730.jpg".to_string(),
        "333c0c32f6dd4d2dfbd867f24ff0b38e285054730.jpg".to_string(),
        "865a5ed4e61b3b7c010d0a2dbff07df1285054730.jpg".to_string(),
        "652193f9df7859da7cea33a7c37569e8285054730.jpg".to_string(),
        "b109e79dab0f243302873e2179690e1a285054730.jpg".to_string(),
        "d098f97837295bd9f5742163913db766285054730.jpg".to_string(),
        "df3856884cc2cc2253b455aa04f62dd8285054730.jpg".to_string(),
        "dffcadea6f4535bcbb37dac155469b1c285054730.jpg".to_string(),
        "e1dd69711908c98771cbfb9cab98e940285054730.jpg".to_string(),
    ];
    // 对每个函数定义一个基准测试
    c.bench_function("manual", |b| b.iter(|| manual(black_box(v.clone()))));

    c.bench_function("winnow", |b| b.iter(|| winnow(black_box(v.clone()))));
}

// 生成基准测试组
criterion_group!(benches, bench_sort);
criterion_main!(benches);
