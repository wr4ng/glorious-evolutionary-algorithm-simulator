export interface Task {
	id: string;
	problem: { type: string };
	tsp_instance: string | undefined;
	algorithm: string;
}
