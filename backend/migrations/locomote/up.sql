CREATE TABLE "card"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"card_id" INT4 NOT NULL,
	"user_id" INT4 NOT NULL,
	"disabled" BOOLEAN NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"balance" FLOAT4 NOT NULL
);

CREATE TABLE "useraccount"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"user_id" INT4 NOT NULL,
	"username" VARCHAR NOT NULL,
	"password_hash" VARCHAR NOT NULL
);

CREATE TABLE "scanner"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"scanner_id" INT4 NOT NULL
);

CREATE TABLE "user"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"user_id" VARCHAR NOT NULL,
	"username" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "ticket"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"card_id" INT4 NOT NULL,
	"user_id" INT4 NOT NULL,
	"origin" VARCHAR NOT NULL,
	"destination" VARCHAR NOT NULL,
	"validity" TIMESTAMP NOT NULL,
	"travelling_at" TIMESTAMP NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
	"cost" FLOAT4 NOT NULL
);

