-- public.author definition

-- Drop table

-- DROP TABLE public.author;

CREATE TABLE public.author (
	id int8 NOT NULL GENERATED ALWAYS AS IDENTITY( INCREMENT BY 1 MINVALUE 1 MAXVALUE 9223372036854775807 START 1 CACHE 1 NO CYCLE),
	"name" varchar NOT NULL,
	CONSTRAINT author_pkey PRIMARY KEY (id)
);


-- public.book definition

-- Drop table

-- DROP TABLE public.book;

CREATE TABLE public.book (
	isbn varchar NOT NULL,
	title varchar NOT NULL,
	author varchar NOT NULL,
	CONSTRAINT book_pkey PRIMARY KEY (isbn)
);


-- public.book_chrono definition

-- Drop table

-- DROP TABLE public.book_chrono;

CREATE TABLE public.book_chrono (
	title varchar NOT NULL,
	published_date date NOT NULL,
	inserted_at timestamptz NULL DEFAULT now()
);


-- public.book_json definition

-- Drop table

-- DROP TABLE public.book_json;

CREATE TABLE public.book_json (
	title varchar NOT NULL,
	metadata json NULL
);


-- public.book_uuid definition

-- Drop table

-- DROP TABLE public.book_uuid;

CREATE TABLE public.book_uuid (
	id uuid NOT NULL,
	title varchar NOT NULL,
	CONSTRAINT book_uuid_pkey PRIMARY KEY (id)
);


-- public.book1 definition

-- Drop table

-- DROP TABLE public.book1;

CREATE TABLE public.book1 (
	isbn varchar NOT NULL,
	title varchar NOT NULL,
	author_id int4 NOT NULL,
	CONSTRAINT book1_pkey PRIMARY KEY (isbn),
	CONSTRAINT book1_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.author(id)
);