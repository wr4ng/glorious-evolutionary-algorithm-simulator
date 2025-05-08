import type { Task } from "../types/task";
import { mapAlgorithmName } from "./algorithm";

export function taskToText(t: Task): string {
	let result = "";
	result += mapAlgorithmName(t.algorithm.type)
	if (t.algorithm.cooling_schedule) {
		if (t.algorithm.cooling_schedule.type == "Static") {
			result += ` (Fixed T = ${t.algorithm.cooling_schedule.temperature})`
		} else if (t.algorithm.cooling_schedule.type == "Exponential") {
			result += ` (c = ${t.algorithm.cooling_schedule.cooling_rate})`
		}
	}
	result += " - " + t.problem.type;
	if (t.problem.type == "OneMax" || t.problem.type == "LeadingOnes") {
		result += ` (n = ${t.problem.bitstring_size})`
	}
	return result;
}
