export interface Task {
	id: string;
	problem: Problem;
	algorithm: Algorithm;
}

export interface TaskResult {
	id: string;
	problem: Problem;
	algorithm: Algorithm;
    final_fitness: number;
    final_iterations: number;
}

export interface Problem {
	type: "OneMax" | "LeadingOnes" | "TSP";
	bitstring_size: number | undefined;
	tsp_instance: string | undefined;
}

export interface Algorithm {
	type: "OnePlusOneEA" | "SimulatedAnnealing" | "ACO";
	cooling_schedule: CoolingSchedule | undefined;
}

export interface CoolingSchedule {
	type: "Static" | "Exponential";
	temperature: number;
	cooling_rate: number;
}
