ALTER TABLE data_records ADD COLUMN voltage2 REAL;
ALTER TABLE data_records DROP COLUMN battery_voltage_1;
ALTER TABLE data_records DROP COLUMN battery_voltage_2;
