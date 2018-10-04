-- oh for crying out loud...
ALTER TABLE data_records ADD COLUMN battery_voltage_1 REAL;
ALTER TABLE data_records ADD COLUMN battery_voltage_2 REAL;
ALTER TABLE data_records DROP COLUMN voltage2;
