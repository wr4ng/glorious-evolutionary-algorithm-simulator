import type { Node } from "../types/types";

//TODO: Maybe wrap in try/catch
export function parseEUC2D(input: string): Node[] {
	const data = input.split("NODE_COORD_SECTION\n", 2)[1];
	const coords = data.split("\nEOF", 1)[0];
	const lines = coords.split("\n");
	return lines.map((l) => {
		const split = l.split(" ");
		let x = parseFloat(split[1]);
		let y = parseFloat(split[2]);
		return { x: x, y: y };
	});
}
