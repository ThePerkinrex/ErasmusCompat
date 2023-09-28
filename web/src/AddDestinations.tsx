import React, { useEffect, useState } from "react";
import {parse} from "csv-parse/browser/esm/sync"

type State = {
	user: string | undefined,
	csv: {data: string, headers: string[]} | undefined,
	csv_delimiter: string,
	has_header: boolean,
}

function AddDestinations() {
	let [state, setState] = useState<State>({
		user: undefined,
		csv: undefined,
		csv_delimiter: ",",
		has_header: false,

	});

	function get_columns(headers: string[], has_header: boolean) {
		return headers.map((header, i) => <option key={header} value={`${i}`}>Column {i}: {has_header ? "" : "[Example] "}{header}</option>)
	}

	return (
		<div className="add-destinations">
			<label htmlFor="add-destinations-user">User</label>{" "}
			<input id="add-destinations-user"></input>
			<br />
			CSV:
			<br />
			<textarea placeholder="CSV data" onInput={(e) => {
				let value: string = (e.target as HTMLInputElement).value
				let records = parse(value, {delimiter: state.csv_delimiter, skip_empty_lines: true, to: 1})
				setState(state => ({...state, csv: {data: value, headers: records[0]}}))
			}}></textarea>
			<br />
			<label htmlFor="add-destinations-csv-delimiter">
				CSV delimiter:
			</label>{" "}
			<input
				id="add-destinations-csv-delimiter"
				value={state.csv_delimiter}
			/>
			<br />
			<label htmlFor="add-destinations-has-header">
				Has header?
			</label>{" "}
			<input
				id="add-destinations-has-header"
				type="checkbox"
				value={state.has_header ? "off" : "on"}
				onInput={(e) =>
					setState((state) => ({
						...state,
						has_header: !state.has_header,
					}))
				}
			/>
			<br />
			{state.csv === undefined ? (
				<span>No hay un CSV</span>
			) : (
				<fieldset>
					<legend>Datos</legend>
					<label htmlFor="add-destinations-erasmus-code">
						ERASMUS CODE:
					</label>{" "}
					<select id="add-destinations-erasmus-code" required>
						<option value="">-- Select a column --</option>
						{get_columns(state.csv.headers, state.has_header)}
					</select>
				</fieldset>
			)}
			<button>Add</button>
		</div>
	);
}

export default AddDestinations;
