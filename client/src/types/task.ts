export interface Task {
	id: string;
	problem: Problem;
	algorithm: Algorithm;
}

export interface Problem {
	type: "OneMax" | "LeadingOnes" | "TSP";
	bitstring_size: number | undefined;
	tsp_instance: string | undefined;
}

export interface Algorithm {
	type: "OnePlusOneEA" | "SimulatedAnnealing" | "ACO";
	tsp_mutator: string | undefined;
}
