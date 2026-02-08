CREATE TABLE "user"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"created_at" TIMESTAMP WITH TIME ZONE DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TRIGGER mdt_moddatetime
    BEFORE UPDATE ON "user"
    FOR EACH ROW
    EXECUTE PROCEDURE moddatetime (updated_at);

CREATE TABLE "useraccount"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"user_id" INT4 NOT NULL REFERENCES "user"(id),
	"username" VARCHAR NOT NULL,
	"password_hash" VARCHAR NOT NULL,
	"created_at" TIMESTAMP WITH TIME ZONE DEFAULT now(),
	"updated_at" TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TRIGGER mdt_moddatetime
    BEFORE UPDATE ON "useraccount"
    FOR EACH ROW
    EXECUTE PROCEDURE moddatetime (updated_at);




CREATE TABLE "card"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"user_id" INT4 NOT NULL REFERENCES "user"(id),
	"card_id" VARCHAR NOT NULL,
	"disabled" BOOLEAN NOT NULL DEFAULT FALSE,
	"created_at" TIMESTAMP WITH TIME ZONE DEFAULT now(),
	"updated_at" TIMESTAMP WITH TIME ZONE DEFAULT now(),
	"balance" FLOAT4 NOT NULL DEFAULT 0.0
);

CREATE TRIGGER mdt_moddatetime
    BEFORE UPDATE ON "card"
    FOR EACH ROW
    EXECUTE PROCEDURE moddatetime (updated_at);



CREATE TABLE "scanner"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"created_at" TIMESTAMP WITH TIME ZONE DEFAULT now(),
	"updated_at" TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TRIGGER mdt_moddatetime
    BEFORE UPDATE ON "scanner"
    FOR EACH ROW
    EXECUTE PROCEDURE moddatetime (updated_at);



CREATE TABLE "ticket"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"card_id" INT4 NOT NULL REFERENCES card(id),
	"user_id" INT4 NOT NULL REFERENCES "user"(id),
	"origin" VARCHAR,
	"destination" VARCHAR,
	"validity" TIMESTAMP WITH TIME ZONE,
	"travelling_at" TIMESTAMP WITH TIME ZONE,
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT now(),
	"updated_at" TIMESTAMP WITH TIME ZONE DEFAULT now(),
	"cost" FLOAT4 NOT NULL
);

CREATE TRIGGER mdt_moddatetime
    BEFORE UPDATE ON "ticket"
    FOR EACH ROW
    EXECUTE PROCEDURE moddatetime (updated_at);