-- Add migration script here
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS "messages" (
	"id"	INTEGER PRIMARY KEY,
	"author"	INTEGER NOT NULL,
	"timestamp"	INTEGER NOT NULL,
	"content"	TEXT NOT NULL,
	"replied_to"	INTEGER,
	FOREIGN KEY("author") REFERENCES "user"("id"),
	FOREIGN KEY("replied_to") REFERENCES "messages"("id")
);

CREATE TABLE IF NOT EXISTS "pin" (
	"msg"	INTEGER NOT NULL UNIQUE,
	"timestamp"	INTEGER NOT NULL,
	FOREIGN KEY("msg") REFERENCES "messages"("id") ON DELETE CASCADE,
	PRIMARY KEY("msg")
);

CREATE TABLE IF NOT EXISTS "token" (
    "token" TEXT NOT NULL UNIQUE,
    "user_id" INTEGER NOT NULL,
	"timestamp" INTEGER NOT NULL,
    PRIMARY KEY("token"),
    FOREIGN KEY("user_id") REFERENCES "user"("id") ON DELETE CASCADE
);

-- id is only for storing the author in messages which saves memory
CREATE TABLE IF NOT EXISTS "user" (
	"id"	INTEGER PRIMARY KEY,
	"username"	TEXT NOT NULL UNIQUE,
	"color"	INTEGER NOT NULL,
	"password"	BLOB NOT NULL,
	"salt" TEXT NOT NULL,
	"status" INTEGER NOT NULL,
	"description" TEXT NOT NULL
);
