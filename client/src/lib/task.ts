import type { Task } from "../types/task";
import { mapAlgorithmName } from "./algorithm";

export function taskToText(t: Task): string {
	let result = "";
	result += mapAlgorithmName(t.algorithm.type)
	// Simulated Annealing
	if (t.algorithm.cooling_schedule) {
		if (t.algorithm.cooling_schedule.type == "Static") {
			result += ` (Fixed T = ${t.algorithm.cooling_schedule.temperature})`
		} else if (t.algorithm.cooling_schedule.type == "Exponential") {
			result += ` (c = ${t.algorithm.cooling_schedule.cooling_rate})`
		}
	}
	// ACO
	if (t.algorithm.type == "ACO") {
		if (t.problem.type == "TSP") {
			result += ` (α=${t.algorithm.alpha} β=${t.algorithm.beta} ρ=${t.algorithm.evap_factor} ants=${t.algorithm.ants})`;
		}
		else {
			result += ` (α=${t.algorithm.alpha} ρ=${t.algorithm.evap_factor} ants=${t.algorithm.ants})`;
		}
	}
	// Problem
	result += " - " + t.problem.type;
	if (t.problem.type == "OneMax" || t.problem.type == "LeadingOnes") {
		result += ` (n = ${t.problem.bitstring_size})`
	}
	if (t.problem.tsp_name) {
		result += ` (${t.problem.tsp_name})`
	}
	return result;
}
