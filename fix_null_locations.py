import csv
import argparse
import os

def parse_args():
	parser = argparse.ArgumentParser()
	parser.add_argument("cities_csv")
	parser.add_argument("unis_csv")
	parser.add_argument("new_cities")
	parser.add_argument("new_unis")
	return parser.parse_args()

def float_or_null(f):
	return "NULL" if f is None else f"{f}"

def parse_or_null(s):
	if s == "NULL":
		return None
	return float(s)


def main(args: argparse.Namespace):
	cities = []
	
	stop = False
	with open(args.cities_csv, 'r', encoding='utf8') as f:
		reader = csv.reader(f)
		for r in reader:
			if len(r) > 0:
				lat = parse_or_null(r[3])
				lon = parse_or_null(r[4])
				if not stop:
					while lat is None or lon is None:
						print(f"[CITY] country: {r[0]}, region: {r[1]}, name: {r[2]}")
						choice = input("Coords can be provided? [y, N, s: stop] ").lower()
						if choice == '' or choice=='n':
							break
						elif choice == 's':
							stop = True
							break
						elif choice == 'y':
							while lat is None:
								try:
									lat_or_latlon = input("latitude or <lat, lon>: ").strip()
									sp = lat_or_latlon.split()
									if len(sp) > 1:
										lat = float(sp[0].strip(" ,").replace(',', '.'))
										lon = float(sp[1].strip(" ,").replace(',', '.'))
									else:
										lat = float(lat_or_latlon.replace(',', '.'))
								except ValueError:
									print("Not a valid float")
							while lon is None:
								try:
									lon = float(input("longitude: ").replace(',', '.'))
								except ValueError:
									lat = None
									print("Not a valid float")
								
				cities.append((r[0], r[1], r[2], float_or_null(lat), float_or_null(lon)))
	unis=[]
	with open(args.unis_csv, 'r', encoding='utf8') as f:
		reader = csv.reader(f)
		for r in reader:
			if len(r) > 0:
				lat = parse_or_null(r[4])
				lon = parse_or_null(r[5])
				if not stop:
					while lat is None or lon is None:
						print(f"[UNI] country: {r[0]}, region: {r[1]}, city: {r[6]}, uni_num: {r[2]}, name: {r[3]}")
						print(f"[UNI] street: {r[7]}, postal code: {r[8]}, website: {r[9]}")
						choice = input("Coords can be provided? [y, N, s: stop] ").lower()
						if choice == '' or choice=='n':
							break
						elif choice == 's':
							stop = True
							break
						elif choice == 'y':
							while lat is None:
								try:
									lat_or_latlon = input("latitude or <lat, lon>: ").strip()
									sp = lat_or_latlon.split()
									if len(sp) > 1:
										lat = float(sp[0].strip(" ,").replace(',', '.'))
										lon = float(sp[1].strip(" ,").replace(',', '.'))
									else:
										lat = float(lat_or_latlon.replace(',', '.'))
								except ValueError:
									print("Not a valid float")
									lat = None
							while lon is None:
								try:
									lon = float(input("longitude: ").replace(',', '.'))
								except ValueError:
									print("Not a valid float")
				unis.append((r[0], r[1], r[2],  r[3], float_or_null(lat), float_or_null(lon), r[6], r[7], r[8], r[9]))
	with open(args.new_cities, "w", encoding='utf8') as f:
		writer = csv.writer(f)
		writer.writerows(cities)
	with open(args.new_unis, "w", encoding='utf8') as f:
		writer = csv.writer(f)
		writer.writerows(unis)
if __name__ == "__main__":
	main(parse_args())
