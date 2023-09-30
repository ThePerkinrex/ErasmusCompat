import React, { useEffect, useState } from "react";
import { parse } from "csv-parse/browser/esm/sync";
import { stringify } from "csv-stringify/browser/esm/sync";

type State = {
	user: string | undefined;
	csv: { table: string[][]; showTable: boolean } | undefined;
	csv_delimiter: string;
	has_header: boolean;
};

function csvTable(table: string[][], has_header: boolean) {
	return (<table>
		{table.map((row, i) => <tr>{ row.map(x => <>{ i === 0 && has_header ? <th>{x}</th> : <td>{x}</td>}</>) }</tr>)}
	</table>)
}

function AddDestinations() {
	let [state, setState] = useState<State>({
		user: undefined,
		csv: undefined,
		csv_delimiter: ",",
		has_header: false,
	});

	function get_columns(headers: string[], has_header: boolean) {
		return headers.map((header, i) => (
			<option key={header} value={`${i}`}>
				Column {i}: {has_header ? "" : "[Example] "}
				{header}
			</option>
		));
	}

	return (
		<div className="add-destinations">
			<label htmlFor="add-destinations-user">User</label>{" "}
			<input id="add-destinations-user"></input>
			<br />
			<label htmlFor="add-destinations-csv-delimiter">
				CSV delimiter:
			</label>{" "}
			<input
				id="add-destinations-csv-delimiter"
				defaultValue={state.csv_delimiter}
			/>
			<br />
			<label htmlFor="add-destinations-has-header">
				Has header?
			</label>{" "}
			<input
				id="add-destinations-has-header"
				type="checkbox"
				defaultValue={state.has_header ? "off" : "on"}
				onInput={(e) =>
					setState((state) => ({
						...state,
						has_header: !state.has_header,
					}))
				}
			/>
			<br />
			<fieldset>
				<legend>
					<button onClick={() => {
						setState(state => {
							if (state.csv !== undefined) {
								return {...state, csv: {...state.csv, showTable: false}}
							}
							return {...state}
						})
						
					}} className={state.csv === undefined || !state.csv.showTable ? "btn-selected" : "btn-unselected"}>CSV</button>
					<button onClick={() => {
						setState(state => {
							if (state.csv !== undefined) {
								return {...state, csv: {...state.csv, showTable: true}}
							}
							return {...state}
						})
					}} className={state.csv === undefined || !state.csv.showTable ? "btn-unselected" : "btn-selected"} disabled={state.csv === undefined}>Table</button>
				</legend>
				{state.csv !== undefined && state.csv.showTable ? (
					csvTable(state.csv.table, state.has_header)
				) : (
					<textarea
						placeholder="CSV data"
						defaultValue={state.csv !== undefined ? stringify(state.csv.table, {delimiter: state.csv_delimiter}) : undefined}
						onInput={(e) => {
							let value: string = (e.target as HTMLInputElement)
								.value;
							try {
								let records: string[][] = parse(value, {
									delimiter: state.csv_delimiter,
									skip_empty_lines: true,
								});
								if (records.length === 0)
									throw new Error("No data");
								setState((state) => ({
									...state,
									csv: { table: records, showTable: false },
								}));
							} catch(e) {
								console.error(e);
								console.log("Set csv to undefined")
								setState((state) => ({
									...state,
									csv: undefined,
								}));
							}
						}}
					></textarea>
				)}
			</fieldset>
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
						{get_columns(state.csv.table[0], state.has_header)}
					</select>
				</fieldset>
			)}
			<button>Add</button>
		</div>
	);
}

export default AddDestinations;
