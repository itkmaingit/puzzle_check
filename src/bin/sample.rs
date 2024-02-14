use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
struct MyObject {
    value: Option<i32>,
}

fn main() {
    // プログレスバーで表示する文字列を指定
    let options_a = [1, 3, 5, 7];
    let options_b = [2, 5, 6];
    let adomain_len = options_a.len();
    let bdomain_len = options_b.len();
    let a_len = 7;
    let b_len = 9;
    let mut count = 0;
    let pb = ProgressBar::new(
        (adomain_len as u64).pow(a_len as u32) * (bdomain_len as u64).pow(b_len as u32),
    );
    // print!(
    //     "{:?}",
    //     (adomain_len as u64).pow(a_len as u32) * (bdomain_len as u64).pow(b_len as u32)
    // );

    let total_combinations_a = adomain_len.pow(a_len); // Aの組み合わせの総数
    let total_combinations_b = bdomain_len.pow(b_len); // Bの組み合わせの総数

    let mut combinations_a = vec![MyObject { value: None }; a_len as usize]; // Aのオブジェクト配列
    let mut combinations_b = vec![MyObject { value: None }; b_len as usize]; // Bのオブジェクト配列

    for i in 0..total_combinations_a {
        let mut index_a = i;

        // Aの組み合わせを生成
        for obj in combinations_a.iter_mut() {
            let digit = index_a % adomain_len;
            index_a /= adomain_len;
            obj.value = Some(options_a[digit]);
        }

        for j in 0..total_combinations_b {
            let mut index_b = j;

            // Bの組み合わせを生成
            for obj in combinations_b.iter_mut() {
                let digit = index_b % bdomain_len;
                index_b /= bdomain_len;
                obj.value = Some(options_b[digit]);
            }

            // ここで条件Pを適用
            // if P(&combinations_a, &combinations_b) { ... }

            // 組み合わせの例を表示
            // println!("A: {:?}, B: {:?}", combinations_a, combinations_b);
            count += 1;
            pb.inc(1);
        }
    }
    println!("{:?}", count);
}
