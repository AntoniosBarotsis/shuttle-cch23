-- Add up migration script here

DROP TABLE IF EXISTS regions;

CREATE TABLE regions (
  id INT PRIMARY KEY,
  name VARCHAR(50)
);
