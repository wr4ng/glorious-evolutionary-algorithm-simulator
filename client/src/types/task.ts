export interface Task {
	id: string;
	problem: Problem;
	algorithm: { type: string };
}

export interface Problem {
	type: "OneMax" | "LeadingOnes" | "TSP";
	bitstring_size: number | undefined;
	tsp_instance: string | undefined;
}
