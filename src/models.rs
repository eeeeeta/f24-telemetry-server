use chrono::NaiveDateTime;
use schema::data_records;

#[derive(Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub current_race: Option<i32>
}
#[derive(Queryable)]
pub struct AccessToken {
    pub id: i32,
    pub team_id: i32,
    pub token: String,
    pub disabled: bool
}
#[derive(Queryable)]
pub struct Car {
    pub id: i32,
    pub team_id: i32,
    pub identifier: String
}
#[derive(Queryable)]
pub struct Race {
    pub id: i32,
    pub team_id: i32,
    pub name: String
}
#[derive(Insertable, Queryable)]
#[table_name="data_records"]
pub struct DataRecord {
    pub car_id: i32,
    pub race_id: i32,
    pub ts: NaiveDateTime,
    pub arduino_secs: Option<i32>,
    pub voltage: Option<f32>,
    pub current: Option<f32>,
    pub watthours: Option<f32>,
    pub temp1: Option<f32>,
    pub temp2: Option<f32>,
    pub gps_speed: Option<f32>,
    pub gps_long: Option<f32>,
    pub gps_lat: Option<f32>,
    pub accel: Option<f32>,
    pub pressure1: Option<f32>,
    pub pressure2: Option<f32>,
    pub rpm1: Option<f32>,
    pub rpm2: Option<f32>,
    pub motor_voltage: Option<f32>,
    pub motor_current: Option<f32>,
    pub gps_track: Option<f32>,
    pub battery_voltage_1: Option<f32>,
    pub battery_voltage_2: Option<f32>,
}
