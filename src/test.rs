use super::Universe;

#[test]
fn sample_test_separated() {
    assert!(1 == 1);
}

#[test]
fn set_universe_size() {
    let uni = Universe::new(30, 30);
    assert_eq!(uni.cells().len(), 900);
}

#[test]
fn set_empty_universe() {
    let uni = Universe::new(10, 10);
    let cells_to_compare: Vec<(u32, u32)> = vec![];
    assert_eq!(uni.get_live_cells(), cells_to_compare);
}

#[test]
fn set_live_cells() {
    let mut uni = Universe::new(10, 10);
    uni.set_live_cells(&vec![(1, 2), (4, 5), (6, 2)]);
    uni.render();
    let live_cells = uni.get_live_cells();
    assert_eq!(live_cells, vec![(1, 2), (4, 5), (6, 2)]);
}

#[test]
fn reset_universe() {
    let mut uni = Universe::new(10, 10);
    uni.set_live_cells(&vec![(1, 2), (3, 4)]);
    uni.reset();
    assert_eq!(uni.get_live_cells().len(), 0);
}

#[test]
fn single_simple_tick() {
    let mut uni = Universe::new(5, 5);
    // simple 3 fields long bar
    uni.set_live_cells(&vec![(1, 2), (2, 2), (3, 2)]);
    uni.tick();
    assert_eq!(uni.get_live_cells(), vec![(2, 1), (2, 2), (2, 3)]);
}

#[test]
fn moving_shape_tick() {
    let mut uni = Universe::new(10, 10);
    uni.set_live_cells(&vec![(1,2), (2,3), (3,1), (3,2), (3,3)]);
    uni.tick();
    assert_eq!(uni.get_live_cells(), vec![(2,1), (2,3), (3,2), (3,3), (4,2)]);
}