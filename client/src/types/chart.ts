export interface DataPoint {
	iteration: number;
	fitness: number;
}

export interface Series {
	data: number[];
	label: string;
	color: string;
	yAxisID: "yfit" | "ytemp";
}

