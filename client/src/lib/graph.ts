import type { Edge } from "../types/types";

export function parsePermutation(input: string): Edge[] {
	let permutation = input.split(",").map(Number);
	let edges: Edge[] = [];
	for (let i = 0; i < (permutation.length - 1); i++) {
		edges.push({ source: permutation[i], target: permutation[i + 1] });
	}
	edges.push({ source: permutation[permutation.length - 1], target: permutation[0] });
	return edges;
}
