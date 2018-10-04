table! {
    access_tokens (id) {
        id -> Int4,
        team_id -> Int4,
        token -> Varchar,
        disabled -> Bool,
    }
}

table! {
    cars (id) {
        id -> Int4,
        team_id -> Int4,
        identifier -> Varchar,
    }
}

table! {
    data_records (id) {
        id -> Int4,
        car_id -> Int4,
        race_id -> Int4,
        ts -> Timestamp,
        arduino_secs -> Nullable<Int4>,
        voltage -> Nullable<Float4>,
        current -> Nullable<Float4>,
        watthours -> Nullable<Float4>,
        temp1 -> Nullable<Float4>,
        temp2 -> Nullable<Float4>,
        gps_speed -> Nullable<Float4>,
        gps_long -> Nullable<Float4>,
        gps_lat -> Nullable<Float4>,
        accel -> Nullable<Float4>,
        pressure1 -> Nullable<Float4>,
        pressure2 -> Nullable<Float4>,
        rpm1 -> Nullable<Float4>,
        rpm2 -> Nullable<Float4>,
        motor_voltage -> Nullable<Float4>,
        motor_current -> Nullable<Float4>,
        gps_track -> Nullable<Float4>,
    }
}

table! {
    races (id) {
        id -> Int4,
        team_id -> Int4,
        name -> Varchar,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
        current_race -> Nullable<Int4>,
    }
}
