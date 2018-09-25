-- Firefly F24 initial schema

-- Teams; own cars, access tokens, and races
CREATE TABLE teams (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	-- The 'current_race' field specifies which race new data will be filed under.
	current_race INT
);
-- Access tokens: used to authenticate a team or a car data entry
CREATE TABLE access_tokens (
	id SERIAL PRIMARY KEY,
	team_id INT NOT NULL REFERENCES teams ON DELETE CASCADE,
	token VARCHAR NOT NULL,
	disabled BOOL NOT NULL DEFAULT false
);
-- Cars: own some data records
CREATE TABLE cars (
	id SERIAL PRIMARY KEY,
	team_id INT NOT NULL REFERENCES teams,
	identifier VARCHAR UNIQUE NOT NULL
);
-- Races: ways of grouping data into separate sections
CREATE TABLE races (
	id SERIAL PRIMARY KEY,
	team_id INT NOT NULL REFERENCES teams,
	name VARCHAR NOT NULL
);
-- Data records!
CREATE TABLE data_records (
	id SERIAL PRIMARY KEY,
	car_id INT NOT NULL REFERENCES cars,
	race_id INT NOT NULL REFERENCES races,
	ts TIMESTAMP NOT NULL,
	arduino_secs INT,
	voltage REAL,
	current REAL,
	watthours REAL,
	temp1 REAL,
	temp2 REAL,
	gps_speed REAL,
	gps_long REAL,
	gps_lat REAL,
	accel REAL,
	pressure1 REAL,
	pressure2 REAL
);
ALTER TABLE TEAMS ADD CONSTRAINT current_race_fk FOREIGN KEY (current_race) REFERENCES races ON DELETE SET NULL;

