-- Your SQL goes here
CREATE TABLE item (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name VARCHAR NOT NULL,
	price INTEGER NOT NULL,
	desc TEXT NOT NULL,
	valid INTEGER NOT NULL DEFAULT 1
);
CREATE TABLE sell (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	item_id INTEGER NOT NULL REFERENCES item (id),
	tz BIGINT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	sold INTEGER
);
CREATE TABLE keymap (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	code SMALLINT NOT NULL,
	key SMALLINT NOT NULL,
	item_id INTEGER NOT NULL REFERENCES item (id)
);
INSERT INTO item (name, price, desc)
	VALUES ("BEEF", 500, "BEEF, ONION, CURRY");
INSERT INTO item (name, price, desc)
	VALUES ("PORK", 500, "PORK, CHIVES");
INSERT INTO item (name, price, desc)
	VALUES ("CHIK", 500, "CHICKEN, CELRY, MUSHROOM");
INSERT INTO item (name, price, desc)
	VALUES ("VEGAN", 500, "CABBAGE, MUSHROOM, SPRING ONION");
-- beef is key 1
INSERT INTO keymap (code, key, item_id)
	VALUES (79,49,1);
-- pork is key 2
INSERT INTO keymap (code, key, item_id)
	VALUES (80,50,2);
-- chicken is key 3
INSERT INTO keymap (code, key, item_id)
	VALUES (81,51,3);
-- vegan is key 4
INSERT INTO keymap (code, key, item_id)
	VALUES (75,52,4);
