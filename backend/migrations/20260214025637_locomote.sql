	CREATE TABLE IF NOT EXISTS usr (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username varchar(32) UNIQUE,
    email varchar(32) UNIQUE,
    password varchar(255), 


    updated_at timestamp WITH TIME ZONE DEFAULT now(),
    created_at timestamp WITH TIME ZONE DEFAULT now(),


    NOT NULL id,
    NOT NULL username,
    NOT NULL email,
    NOT NULL password,
    NOT NULL updated_at,
    NOT NULL created_at
);

CREATE TABLE IF NOT EXISTS card (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id integer REFERENCES usr(id),

    code varchar(32),

    updated_at timestamp WITH TIME ZONE DEFAULT now(),
    created_at timestamp WITH TIME ZONE DEFAULT now(),
    NOT NULL id,
    NOT NULL user_id,
    NOT NULL code,
    NOT NULL updated_at,
    NOT NULL created_at
);

CREATE TABLE IF NOT EXISTS scanner {
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
}

CREATE TABLE IF NOT EXISTS ticket (
    id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id integer REFERENCES usr(id),
    card_id integer REFERENCES card(id),
    scanner_id integer REFERENCES scanner(id)
    origin text,
    destination text,

    duration interval,

    arrival timestamp WITH TIME ZONE,
    boarding timestamp WITH TIME ZONE,

    updated_at timestamp WITH TIME ZONE DEFAULT now(),
    created_at timestamp WITH TIME ZONE DEFAULT now(),


    NOT NULL id,
    NOT NULL user_id,
    NOT NULL card_id,
    NOT NULL origin,
    NOT NULL destination,
    NOT NULL duration,
    NOT NULL arrival,
    NOT NULL boarding,
    NOT NULL updated_at,
    NOT NULL created_at
);

CREATE EXTENSION IF NOT EXISTS moddatetime;
CREATE OR REPLACE TRIGGER mdt_moddatetime BEFORE UPDATE ON usr FOR EACH ROW EXECUTE PROCEDURE moddatetime (updated_at);
CREATE OR REPLACE TRIGGER mdt_moddatetime BEFORE UPDATE ON card FOR EACH ROW EXECUTE PROCEDURE moddatetime (updated_at);
CREATE OR REPLACE TRIGGER mdt_moddatetime BEFORE UPDATE ON ticket FOR EACH ROW EXECUTE PROCEDURE moddatetime (updated_at);
