-- Add up migration script here

CREATE TABLE Pais(
	codigo TEXT NOT NULL,
	nombre TEXT NOT NULL,
	codigo_iso TEXT NOT NULL,
	PRIMARY KEY(codigo)
);

CREATE TABLE Ciudad(
	region TEXT NOT NULL,
	pais TEXT NOT NULL,
	nombre TEXT NOT NULL,
	lat FLOAT,
	lon FLOAT,
	PRIMARY KEY(region, pais, nombre),
	FOREIGN KEY(pais) REFERENCES Pais(codigo)
);

CREATE TABLE Universidad(
	numero INT NOT NULL,
	region TEXT NOT NULL,
	ciudad TEXT NOT NULL,
	pais TEXT NOT NULL,
	nombre TEXT,
	lat FLOAT,
	lon FLOAT,
	direccion TEXT NOT NULL,
	postal TEXT NOT NULL,
	webpage TEXT NOT NULL,
	PRIMARY KEY(numero, region, ciudad, pais),
	FOREIGN KEY(pais) REFERENCES Pais(codigo),
	FOREIGN KEY(region, ciudad, pais) REFERENCES Ciudad(region, nombre, pais)
);

CREATE TABLE Persona(
	persona TEXT NOT NULL,
	PRIMARY KEY(persona)
);

CREATE TABLE Destino(
	persona TEXT NOT NULL,
	universidad INT NOT NULL,
	region TEXT NOT NULL,
	ciudad TEXT NOT NULL,
	pais TEXT NOT NULL,
	PRIMARY KEY(persona, universidad, region, ciudad, pais),
	FOREIGN KEY(pais) REFERENCES Pais(codigo),
	FOREIGN KEY(region, ciudad, pais) REFERENCES Ciudad(region, nombre, pais),
	FOREIGN KEY(universidad, region, ciudad, pais) REFERENCES Universidad(numero, region, ciudad, pais),
	FOREIGN KEY(persona) REFERENCES Persona(persona)
);

CREATE TABLE OpcionDestino(
	opcion INT NOT NULL,
	persona TEXT NOT NULL,
	universidad INT NOT NULL,
	region TEXT NOT NULL,
	ciudad TEXT NOT NULL,
	pais TEXT NOT NULL,
	plazas INT,
	meses INT,
	nivel_estudios TEXT,
	observaciones TEXT,
	idioma TEXT,
	PRIMARY KEY(opcion, persona, universidad, region, ciudad, pais),
	FOREIGN KEY(pais) REFERENCES Pais(codigo),
	FOREIGN KEY(region, ciudad, pais) REFERENCES Ciudad(region, nombre, pais),
	FOREIGN KEY(universidad, region, ciudad, pais) REFERENCES Universidad(numero, region, ciudad, pais),
	FOREIGN KEY(persona) REFERENCES Persona(persona),
	FOREIGN KEY(persona, universidad, region, ciudad, pais) REFERENCES Destino(persona, universidad, region, ciudad, pais)
)
