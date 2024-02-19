use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
struct MyObject {
    value: Option<i32>,
}

fn main() {
    // プログレスバーで表示する文字列を指定
    let a_domain = [1, 3];
    let b_domain = [2, 5];
    let adomain_len = a_domain.len();
    let bdomain_len = b_domain.len();
    let p_size = 9;
    let c_size = 4;
    let mut count = 0;
    let pb = ProgressBar::new(
        (adomain_len as u64).pow(p_size as u32) * (bdomain_len as u64).pow(c_size as u32),
    );
    // print!(
    //     "{:?}",
    //     (adomain_len as u64).pow(p_size as u32) * (bdomain_len as u64).pow(c_size as u32)
    // );

    let total_combinations_a = adomain_len.pow(p_size); // Aの組み合わせの総数
    let total_combinations_b = bdomain_len.pow(c_size); // Bの組み合わせの総数

    let mut combinations_a = vec![MyObject { value: None }; p_size as usize]; // Aのオブジェクト配列
    let mut combinations_b = vec![MyObject { value: None }; c_size as usize]; // Bのオブジェクト配列

    for i in 0..total_combinations_a {
        let mut index_a = i;

        for obj in combinations_a.iter_mut() {
            let digit = index_a % adomain_len;
            index_a /= adomain_len;
            obj.value = Some(a_domain[digit]);
        }

        for j in 0..total_combinations_b {
            let mut index_b = j;

            // Bの組み合わせを生成
            for obj in combinations_b.iter_mut() {
                let digit = index_b % bdomain_len;
                index_b /= bdomain_len;
                obj.value = Some(b_domain[digit]);
            }

            // ここで条件Pを適用
            // if P(&combinations_a, &combinations_b) { ... }

            // 組み合わせの例を表示
            println!("A: {:?}, B: {:?}", combinations_a, combinations_b);
            // count += 1;
            pb.inc(1);
        }
    }
    // println!("{:?}", count);
}
