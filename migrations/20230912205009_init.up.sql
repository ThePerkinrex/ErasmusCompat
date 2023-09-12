-- Add up migration script here

CREATE TABLE Pais(
	codigo TEXT,
	nombre TEXT,
	PRIMARY KEY(codigo)
);

CREATE TABLE Ciudad(
	codigo TEXT,
	pais TEXT,
	nombre TEXT,
	PRIMARY KEY(codigo, pais),
	FOREIGN KEY(pais) REFERENCES Pais(codigo)
);

CREATE TABLE Universidad(
	numero INT,
	ciudad TEXT,
	pais TEXT,
	nombre TEXT,
	
	PRIMARY KEY(numero, ciudad, pais),
	FOREIGN KEY(pais) REFERENCES Pais(codigo),
	FOREIGN KEY(ciudad, pais) REFERENCES Ciudad(ciudad, pais)
);

CREATE TABLE Persona(
	persona TEXT,
	PRIMARY KEY(persona)
);

CREATE TABLE Destino(
	persona TEXT,
	universidad INT,
	ciudad TEXT,
	pais TEXT,
	PRIMARY KEY(persona, universidad, ciudad, pais),
	FOREIGN KEY(pais) REFERENCES Pais(codigo),
	FOREIGN KEY(ciudad, pais) REFERENCES Ciudad(ciudad, pais),
	FOREIGN KEY(universidad, ciudad, pais) REFERENCES Universidad(numero, ciudad, pais),
	FOREIGN KEY(persona) REFERENCES Persona(nombre)
);

CREATE TABLE OpcionDestino(
	opcion INT,
	persona TEXT,
	universidad INT,
	ciudad TEXT,
	pais TEXT,
	plazas INT,
	meses INT,
	nivel_estudios TEXT,
	observaciones TEXT,
	idioma TEXT,
	PRIMARY KEY(opcion, persona, universidad, ciudad, pais),
	FOREIGN KEY(pais) REFERENCES Pais(codigo),
	FOREIGN KEY(ciudad, pais) REFERENCES Ciudad(ciudad, pais),
	FOREIGN KEY(universidad, ciudad, pais) REFERENCES Universidad(numero, ciudad, pais),
	FOREIGN KEY(persona) REFERENCES Persona(nombre),
	FOREIGN KEY(persona, universidad, ciudad, pais) REFERENCES Destino(persona, universidad, ciudad, pais)
)
