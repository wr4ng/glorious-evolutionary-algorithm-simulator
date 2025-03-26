<script lang="ts">
	const resolution = 100;

	interface Point {
		x: number;
		y: number;
	}

	interface OnionProps {
		pointData: Point[];
	}

	const { pointData }: OnionProps = $props();

	//TODO: Determine values for x-range (vertical)
	const xDiff = 3.5;

	// Gaussian function with mu=0 and sigma=1
	function gaussian(x: number) {
		return Math.exp(-(x ** 2) / 2); // e^(-x^2 / 2)
	}

	// Calculate gaussian value for resolution points from minX to maxX (in gaussion coordinate system, standard xy)
	const gaussPoints: Point[] = [{ x: -xDiff, y: 0 }];
	for (let x = -xDiff; x <= xDiff; x += 2 * xDiff / resolution) {
		const y = gaussian(x);
		gaussPoints.push({ x: x, y: y });
	}
	gaussPoints.push({ x: xDiff, y: 0 });

	function toPath(points: Point[]) {
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
	function mapPercentagePoint(p: Point) {
		// Compute gaussian value at given percentage
		const gaussX = -xDiff + 2 * xDiff * p.y;
		const gaussY = gaussian(gaussX);

		// Map y percentage to [0; 100] + flip direction
		const py = 100 - p.y * 100;

		// Calculate x distance to point from center, in the gaussian coordinate space
		const xDistance = (p.x - 0.5) * gaussY;
		// Map distance to view space
		const px = 50 - 100 * xDistance;

		return { x: px, y: py };
	}

	const pointDataView = pointData.map((p) => mapPercentagePoint(p));

	// Map a point from gaussian coordinate space to view space
	function gaussToView(p: Point): Point {
		// p.x is [0; 1]. Map to distance of y-axis (half total distance)
		const x = 50 - p.x * 50;
		// Map p.x value from [-xDiff; xDiff] to [0; 100], then flip it (so 0 is at bottom)
		// [-xDiff; xDiff] -> [0; 2 * xDiff] -> [0; 1] -> [0; 100]
		const y = 100 - ((p.y + xDiff) / (2 * xDiff)) * 100;
		return { x, y };
	}
</script>

<div style="width: 100%; height: 100%;">
	<svg
		width="100%"
		height="100%"
		viewBox="-5 -5 110 110"
		preserveAspectRatio="xMidYMid meet"
	>
		<line
			x1="50"
			y1="1"
			x2="50"
			y2="100"
			stroke="black"
			stroke-opacity="0.3"
			stroke-width="2"
			stroke-dasharray="5,5"
			vector-effect="non-scaling-stroke"
		/>
		<line
			x1="0"
			y1="50"
			x2="100"
			y2="50"
			stroke="black"
			stroke-width="2"
			stroke-opacity="0.3"
			stroke-dasharray="5,5"
			vector-effect="non-scaling-stroke"
		/>
		<path
			d={toPath(gaussPoints)}
			stroke="red"
			fill="none"
			stroke-width="3"
			vector-effect="non-scaling-stroke"
		/>
		{#each pointDataView as p}
			<circle cx={p.x} cy={p.y} r="1" class="node" />
		{/each}
	</svg>
</div>

<style>
	.node {
		fill: blue;
	}
</style>
