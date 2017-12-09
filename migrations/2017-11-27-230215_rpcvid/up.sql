-- Your SQL goes here
CREATE TABLE vids0 (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  len BIGINT NOT NULL DEFAULT 0,
  duration real not null DEFAULT 0.0,
  bit_rate INT NOT NULL DEFAULT 1,
  width SMALLINT DEFAULT 1,
  height SMALLINT DEFAULT 1
);

CREATE TABLE vidsinfo0 (
  id INT REFERENCES vids0 (id),
  access TIMESTAMP DEFAULT now(),
  modify TIMESTAMP DEFAULT now(),
  viewed INT DEFAULT 0,
  rate SMALLINT DEFAULT -1,
  quality SMALLINT DEFAULT -1,
  accumtime INT DEFAULT 0,
  fpath VARCHAR,
  hash VARCHAR,
  PRIMARY KEY (id)
);

CREATE VIEW vidfull AS
    SELECT t1.name, t1.len, t1.duration, t1.bit_rate, t1.width, t1.height, t2.*
    FROM vids0 t1 JOIN vidsinfo0  t2 ON ( t1.id = t2.id);
