use conditionally_trait_bounds as ctb;

fn main() {

    let pair = ctb::Pair::new(5,7);
    pair.cmp_display();
    let pair = ctb::Pair::new('z','a');
    pair.cmp_display();
}
