CREATE TABLE items (
	id serial4 NOT NULL,
	category int4 NOT NULL,
	kcal float8 NOT NULL,
	protein float8 NOT NULL,
	carbo float8 NOT NULL,
	lipids float8 NOT NULL,
	fibers float8 NOT NULL,
	"name" text NOT NULL,
	CONSTRAINT items_pkey PRIMARY KEY (id)
);
