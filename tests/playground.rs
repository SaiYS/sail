#[test]
fn feature() {
    let mut st = segment_tree::SegmentTree::<algebraic_structures::monoid::MSum<usize>>::new(10);
    for i in 0..10 {
        st.update(i, i);
    }

    st.dbg_tree()
}
