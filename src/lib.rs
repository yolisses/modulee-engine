mod get_items_by_id;
mod get_updated_group;
pub mod graph;
mod group;
mod groups_by_id;
mod node;
mod node_trait;
mod nodes;
mod sample_rate;
mod sort;
mod values_by_id;
mod voice;

pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
