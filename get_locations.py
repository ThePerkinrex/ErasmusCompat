import csv
import argparse
import pycountry
import re
import requests
import time
import os

def parse_args():
	parser = argparse.ArgumentParser()
	parser.add_argument("unis_file", type=argparse.FileType('r', encoding="utf8"))
	parser.add_argument("countries_csv")
	parser.add_argument("cities_csv")
	parser.add_argument("unis_csv")
	parser.add_argument('-n', type=int)
	parser.add_argument('--total', type=int)
	return parser.parse_args()

def get_place(location):
	# print("Searching for " + location)
	headers = {
		'User-Agent': 'ErasmusLocator/0.1.0'
	}
	time.sleep(1)
	r = requests.get('https://nominatim.openstreetmap.org/search', params={'q': location, 'format': 'json'}, headers=headers).json()
	if len(r) > 0:
		return r[0]
	return None

def float_or_null(f):
	return "NULL" if f is None else f"{f}"

def parse_or_null(s):
	if s == "NULL":
		return None
	return float(s)

def main(args: argparse.Namespace):
	def print_elem(i, *e):
		if args.total is not None:
			print(i, f"{round(i/args.total * 100, 2)}%", *e)
		else:
			print(i, *e)
	reader = csv.reader(args.unis_file, delimiter=';')
	next(reader)
	countries = {}
	cities = {}
	universities = {}
	replacements = {'EL': 'GR', 'UK': 'GB'}
	names_exc = {'XK': 'Kosovo'}
	if os.path.exists(args.countries_csv):
		with open(args.countries_csv, encoding='utf8') as f:
			f_r = csv.reader(f)
			for r in f_r:
				if len(r) > 0:
					countries[r[0]] = {"name": r[1], "code": r[2]}
	if os.path.exists(args.cities_csv):
		with open(args.cities_csv, encoding='utf8') as f:
			f_r = csv.reader(f)
			for r in f_r:
				if len(r) > 0:
					cities[(r[0], r[1], r[2])] = {"lat": parse_or_null(r[3]), "lon": parse_or_null(r[4])}
	
	if os.path.exists(args.unis_csv):
		with open(args.unis_csv, encoding='utf8') as f:
			f_r = csv.reader(f)
			for r in f_r:
				if len(r) > 0:
					universities[(r[0], r[1], r[2])] = {"name": r[3], "lat": parse_or_null(r[4]), "lon": parse_or_null(r[5]),"city": r[6], "street": r[7], "postal_code": r[8], "webpage": r[9]}
	try:
		not_skipped = 0
		for i,row in enumerate(reader):
			skipped=True
			country = None
			uni_city = None
			try:
				[country, uni_city] = row[1].split()
			except:
				country = row[1][:3]
				uni_city = row[1][3:]
			if country not in countries:
				code = replacements.get(row[8], row[8])
				name = names_exc.get(code)
				if name is None:
					name = pycountry.countries.get(alpha_2=code).name
				countries[country] = {"code": code, "name": name}
				skipped=False
			else:
				print(f"Skipping country {country}")
			split_re = re.match(r"([A-Z-]+)(\d+)", uni_city)
			uni = split_re.group(2)
			city = split_re.group(1)
			city_key = (country, city, row[7].title())
			if city_key not in cities:
				place = get_place(f"{row[7]}, {countries[country]['name']}")
				if place is None:
					place = {"lat": None, "lon": None}
				else:
					place['lat'] = float(place['lat'])
					place['lon'] = float(place['lon'])
				cities[city_key] = {"lat": place['lat'], "lon": place['lon']}
				print_elem(i, city_key, cities[city_key])
				skipped=False
			else:
				print(f"Skipping city {city_key}")
			uni_key = (country, city, uni)
			if uni_key not in universities:
				place = get_place(f"{row[5]}, {row[6]} {row[7]}, {countries[country]['name']}")
				if place is None:
					print("Second get")
					place = get_place(f"{row[4]}, {row[6]} {row[7]}, {countries[country]['name']}")
				if place is None:
					print("Third get")
					place = get_place(f"{row[4]}, {row[5]}, {row[6]} {row[7]}, {countries[country]['name']}")
				if place is None:
					place = {"lat": None, "lon": None}
				else:
					place['lat'] = float(place['lat'])
					place['lon'] = float(place['lon'])
				universities[uni_key] = {"name": row[4], "city": row[7].title(), "street": row[5], "postal_code": row[6], "webpage": row[9], "lat": place['lat'], "lon": place['lon']}
				print_elem(i, uni_key, universities[uni_key])
				skipped=False
			else:
				print(f"Skipping uni {uni_key}")
			if args.n is not None and not_skipped == args.n:
				break
			if not skipped:
				not_skipped+=1
	except KeyboardInterrupt:
		print("CtrlC")
	except Exception as e:
		print("Error: " + e)
	with open(args.countries_csv, "w", encoding='utf8') as f:
		writer = csv.writer(f)
		writer.writerows(map(lambda x: (x, countries[x]['name'], countries[x]['code']), countries))
		# for country in countries:
		# 	c = countries[country]
			
		# 	args.up_file.write(f"INSERT INTO Pais(codigo, nombre, codigo_iso) VALUES('{country}', '{c['name']}', '{c['code']}');\n")
	with open(args.cities_csv, "w", encoding='utf8') as f:
		writer = csv.writer(f)
		writer.writerows(map(lambda x: (x[0], x[1], x[2], float_or_null(cities[x]['lat']), float_or_null(cities[x]['lon'])), cities))
	# for city in cities:
	# 	c = cities[city]
	# 	args.up_file.write(f"INSERT INTO Ciudad(pais, codigo, nombre, lat, lon) VALUES('{city[0]}', '{city[1]}', '{c['name']}', {float_or_null(c['lat'])}, {float_or_null(c['lon'])});\n")
	with open(args.unis_csv, "w", encoding='utf8') as f:
		writer = csv.writer(f)
		writer.writerows(map(lambda x: (x[0], x[1], x[2], universities[x]['name'], float_or_null(universities[x]['lat']), float_or_null(universities[x]['lon']), universities[x]['city'], universities[x]['street'], universities[x]['postal_code'], universities[x]['webpage']), universities))
	# for uni in universities:
	# 	c = universities[uni]
	# 	args.up_file.write(f"INSERT INTO Universidad(pais, ciudad, numero, nombre, lat, lon, direccion, postal, webpage) VALUES('{uni[0]}', '{uni[1]}', {uni[2]}, '{c['name']}', {float_or_null(c['lat'])}, {float_or_null(c['lon'])}, '{c['street']}', '{c['postal_code']}', '{c['webpage']}');\n")
	

if __name__ == "__main__":
	main(parse_args())
