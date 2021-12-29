//Math from here: https://clintbellanger.net/articles/isometric_math/

//Convert world coordinates to isometric coordinates given a tile size
pub fn to_isometric(world_x: f32, world_y: f32, tile_width: f32, tile_height: f32) -> (f32, f32) {
    let x = world_x.floor();
    let y = world_y.floor();

    let iso_x = (((x / (tile_width / 2f32)) + (y / (tile_height / 2f32))) / 2f32).floor();
    let iso_y = (((y / (tile_height / 2f32)) - (x / (tile_width / 2f32))) / 2f32).floor();

    (iso_x, iso_y)
}

//Convert isometric coordinates to world coordinates given a tile size
pub fn to_world(iso_x: f32, iso_y: f32, tile_width: f32, tile_height: f32) -> (f32, f32) {
    let world_x = (iso_x - iso_y) * tile_width / 2f32;
    let world_y = (iso_x + iso_y) * tile_height / 2f32;

    (world_x, world_y)
}

#[cfg(test)]
mod test_to_iso {
    use super::*;

    #[test]
    fn to_isometric_test_origo() {
        //Using tile size 64x32. Asserting that world (0,0) == iso (0,0)
        let coordinates = to_isometric(0f32,0f32,64f32,32f32);
        assert_eq!((0f32,0f32), coordinates);
    }
    #[test]
    fn to_isometric_test_64_by_32() {
        //Using tile size 64x32. Asserting that world (32,16) == iso (1,0)
        let coordinates = to_isometric(32f32, 16f32, 64f32, 32f32);
        assert_eq!((1f32, 0f32), coordinates);
    }
    #[test]
    fn to_isometric_test_128_by_64() {
        //Using tile size 128x64. Asserting that world (64,96) == iso (2,1)
        let coordinates = to_isometric(64f32,96f32,128f32,64f32);
        assert_eq!((2f32, 1f32), coordinates);
    }
}
#[cfg(test)]
mod test_to_world {
    use super::*;
    #[test]
    fn to_world_test_origo() {
        //Using tile size 64x32. Asserting that world (0,0) == iso (0,0)
        let coordinates = to_world(0f32, 0f32, 64f32, 32f32);
        assert_eq!((0f32, 0f32), coordinates);
    }
    #[test]
    fn to_world_test_1_point_zero() {
        //Using tile size 64x32. Asserting that world (32,16) == iso (1,0)
        let coordinates = to_world(1f32, 0f32, 64f32, 32f32);
        assert_eq!((32f32, 16f32), coordinates);
    }
    #[test]
    fn to_world_test_128_by_64() {
        //Using tile size 128x64. Asserting that world (64,96) == iso (2,1)
        let coordinates = to_world(2f32, 1f32, 128f32, 64f32);
        assert_eq!((64f32, 96f32), coordinates);
    }
}
