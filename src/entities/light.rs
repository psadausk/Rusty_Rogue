//A light source. Can be anything such as torch, fire, flashlight
//Illumination is calculated as:
//  100% if distance is below falloff distance
//   1/((fall_off_distance - distance)^2)
//Color is determined by (rgb) * (1- illumination)

const MAX_SHADE: f32 = 0.8;
pub struct LightSource {
    pub intensity: f32,         //Brightness
    pub fall_off_distance: f32, //Amount of tiles light illuminates before starting to drop off
    //pub fade_factor: f32,       //How sharply the color drops off
    pub angle: i32,    //For now assume 360
    pub max_dist: f32, //Max distance light can cover. A field calculated once on new
}

impl LightSource {
    pub fn new(fall_off_distance: f32, intensity: f32) -> Self {
        return LightSource {
            angle: 360,
            fall_off_distance,
            intensity,
            max_dist: (MAX_SHADE * intensity + fall_off_distance).ceil() as f32,
        };
    }

    //Number between 0, and 1 that calcs the percent of shade that covers the tile.
    pub fn calc_shade_percent(&self, distance: f32) -> f32 {
        if distance <= self.fall_off_distance {
            return 0.0;
        } else {
            return (distance - self.fall_off_distance) / (self.intensity);
        }
    }
}
