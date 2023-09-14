import csv
import argparse
import os

def parse_args():
	parser = argparse.ArgumentParser()
	parser.add_argument("countries_csv", type=argparse.FileType('r', encoding="utf8"))
	parser.add_argument("cities_csv", type=argparse.FileType('r', encoding="utf8"))
	parser.add_argument("unis_csv", type=argparse.FileType('r', encoding="utf8"))
	parser.add_argument("migration", type=argparse.FileType('w', encoding="utf8"))
	return parser.parse_args()

def escape(s: str):
	print(s + " -> " + s.replace("'", "''"))
	return s.replace("'", "''")

def main(args: argparse.Namespace):
	with args.migration as migration:
		with args.countries_csv as f:
			reader = csv.reader(f)
			for r in reader:
				if len(r) > 0:
					print(r)
					migration.write(f"INSERT INTO Pais(codigo, nombre, codigo_iso) VALUES('{r[0]}', '{r[1]}', '{r[2]}');\n")
		migration.write(f"\n")
		with args.cities_csv as f:
			reader = csv.reader(f)
			for r in reader:
				if len(r) > 0:
					migration.write(f"INSERT INTO Ciudad(pais, region, nombre, lat, lon) VALUES('{r[0]}', '{r[1]}', '{escape(r[2])}', {r[3]}, {r[4]});\n")
		migration.write(f"\n")
		with args.unis_csv as f:
			reader = csv.reader(f)
			for r in reader:
				if len(r) > 0:
					migration.write(f"INSERT INTO Universidad(pais, region, ciudad, numero, nombre, lat, lon, direccion, postal, webpage) VALUES('{r[0]}', '{r[1]}', '{escape(r[6])}', {r[2]}, '{escape(r[3])}', {r[4]}, {r[5]}, '{escape(r[7])}', '{escape(r[8])}', '{escape(r[9])}');\n")
if __name__ == "__main__":
	main(parse_args())
