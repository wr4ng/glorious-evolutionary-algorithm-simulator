import type { OnionPoint, Point } from "../types/onion.ts";

// Gaussian function with mu=0 and sigma=1
function gaussian(x: number) {
	return Math.exp(-(x ** 2) / 2); // e^(-x^2 / 2)
}

const xDiff = 3.5;
const resolution = 100;

// Calculate gaussian value for resolution points from minX to maxX (in gaussion coordinate system, standard xy)
export function generateGaussPoints(): Point[] {
	const gaussPoints = [{ x: -xDiff, y: 0 }];
	for (let x = -xDiff; x <= xDiff; x += (2 * xDiff) / resolution) {
		const y = gaussian(x);
		gaussPoints.push({ x: x, y: y });
	}
	gaussPoints.push({ x: xDiff, y: 0 });
	return gaussPoints;
}

// Map a point from gaussian coordinate space to view space
function gaussToView(p: Point): Point {
	// p.x is [0; 1]. Map to distance of y-axis (half total distance)
	const x = 50 - p.x * 50;
	// Map p.x value from [-xDiff; xDiff] to [0; 100], then flip it (so 0 is at bottom)
	// [-xDiff; xDiff] -> [0; 2 * xDiff] -> [0; 1] -> [0; 100]
	const y = 100 - ((p.y + xDiff) / (2 * xDiff)) * 100;
	return { x, y };
}

export function generateGaussPath(points: Point[]) {
	const leftPath = points
		.map((p, i) => {
			const { x, y } = gaussToView({ x: p.y, y: p.x });
			return `${i === 0 ? "M" : "L"} ${x} ${y}`;
		})
		.join(" ");

	const rightPath = points
		.map((p, i) => {
			const { x, y } = gaussToView({ x: -p.y, y: p.x });
			return `${i === 0 ? "M" : "L"} ${x} ${y}`;
		})
		.join(" ");

	return leftPath + " " + rightPath;
}

// Maps a percentage point {x: [0; 1], y: [0; 1]} to view coordinates
export function mapPercentageToView(p: OnionPoint): OnionPoint {
	// Compute gaussian value at given percentage
	const gaussX = -xDiff + 2 * xDiff * p.y;
	const gaussY = gaussian(gaussX);

	// Map y percentage to [0; 100] + flip direction
	const py = 100 - p.y * 100;

	// Calculate x distance to point from center, in the gaussian coordinate space
	const xDistance = (p.x - 0.5) * gaussY;
	// Map distance to view space
	const px = 50 - 100 * xDistance;

	return { x: px, y: py, tooltip: p.tooltip};
}

export function bitstringToOnionCoords(bitstring: string, tooltip: string): OnionPoint {
	const numOnes = (bitstring.match(/1/g) || []).length;
	if (numOnes == bitstring.length) {
		return { x: 1, y: 1, tooltip: tooltip };
	} else if (numOnes == 0) {
		return { x: 0, y: 0, tooltip: tooltip};
	}
	const vertical = numOnes / bitstring.length;

	let averageOneIndex = 0;
	for (let i = 0; i < bitstring.length; i++) {
		if (bitstring[i] == "1") {
			averageOneIndex += bitstring.length - 1 - i;
		}
	}
	let minAverage = 0;
	let maxAverage = 0;
	for (let i = 0; i < numOnes; i++) {
		minAverage += i;
		maxAverage += bitstring.length - 1 - i;
	}
	// Map averageOneIndex from [minAverage; maxAverage] to [0; 1]
	let horizontal =
		(averageOneIndex - minAverage) / (maxAverage - minAverage);

	return { x: horizontal, y: vertical, tooltip: tooltip };
}
