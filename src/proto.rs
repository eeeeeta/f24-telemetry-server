use models::DataRecord;
use chrono::NaiveDateTime;

#[derive(FromForm)]
pub struct AuthDetails {
    pub access_token: String
}
#[derive(Deserialize)]
pub struct UploadBody {
    pub ts: u64,
    #[serde(default)]
    pub arduino_secs: Option<i32>,
    #[serde(default)]
    pub voltage: Option<f32>,
    #[serde(default)]
    pub current: Option<f32>,
    #[serde(default)]
    pub watthours: Option<f32>,
    #[serde(default)]
    pub temp1: Option<f32>,
    #[serde(default)]
    pub temp2: Option<f32>,
    #[serde(default)]
    pub gps_speed: Option<f32>,
    #[serde(default)]
    pub gps_long: Option<f32>,
    #[serde(default)]
    pub gps_lat: Option<f32>,
    #[serde(default)]
    pub accel: Option<f32>,
    #[serde(default)]
    pub pressure1: Option<f32>,
    #[serde(default)]
    pub pressure2: Option<f32>,
    #[serde(default)]
    pub rpm1: Option<f32>,
    #[serde(default)]
    pub rpm2: Option<f32>,
}
impl UploadBody {
    pub fn as_data_record(self) -> DataRecord {
        DataRecord {
            car_id: -1,
            race_id: -1,
            ts: NaiveDateTime::from_timestamp(0, 0),
            arduino_secs: self.arduino_secs,
            voltage: self.voltage,
            current: self.current,
            watthours: self.watthours,
            temp1: self.temp1,
            temp2: self.temp2,
            gps_speed: self.gps_speed,
            gps_long: self.gps_long,
            gps_lat: self.gps_lat,
            accel: self.accel,
            pressure1: self.pressure1,
            pressure2: self.pressure2,
            rpm1: self.pressure2,
            rpm2: self.pressure2,
        }
    }
}
