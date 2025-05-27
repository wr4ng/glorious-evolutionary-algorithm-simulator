export interface Task {
	problem: Problem;
	algorithm: Algorithm;
	stop_cond: StopCondition;
}

export interface TaskResult {
	task: Task;
	fitness: number;
	iterations: number;
}

export interface Problem {
	type: string; //"OneMax" | "LeadingOnes" | "TSP";
	bitstring_size?: number;
	tsp_instance?: string;
	tsp_name?: string;
}

export interface Algorithm {
	type: string; // "OnePlusOneEA" | "SimulatedAnnealing" | "ACO";
	cooling_schedule?: CoolingSchedule;
	alpha?: number;
	beta?: number;
	evap_factor?: number;
	ants?: number;
	p_best?: number;
	q?: number;
	nn?: boolean;
	update_strategy?: string;
}

export interface CoolingSchedule {
	type: string; // "Static" | "Exponential";
	temperature?: number;
	cooling_rate?: number;
}

export interface StopCondition {
	max_iterations: number;
	optimal_fitness?: number;
}

export interface TaskSchedule {
	id: string;
	seed: number;
	tasks: Task[];
}

export interface TaskScheduleRequest {
	tasks: Task[];
	repeat_count: number;
	update_rate: number;
	seed: number;
}

export interface TaskScheduleResult {
	results: TaskResult[];
}
