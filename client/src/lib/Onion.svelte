<script lang="ts">
	const resolution = 100;

	//TODO: Take as props
	const pointData: Point[] = [
		{ x: 0.5, y: 0.5 },
		{ x: 0.2, y: 0.6 },
		{ x: 0.8, y: 0.7 },
		{ x: 1, y: 0.7 },
		{ x: 1, y: 1 },
		{ x: 1, y: 0 },
	];

	// Gaussian function with mu=0 and sigma=1
	function gaussian(x: number) {
		// e^(-x^2 / 2)
		return Math.exp(-(x ** 2) / 2);
	}

	//TODO: Determine values for x-range (vertical)
	const minX = -3.5;
	const maxX = 3.5;
	const minY = 0; // gaussian(0) = e^(-0^2/2) = 1;
	const maxY = 1;

	interface Point {
		x: number;
		y: number;
	}

	const points: Point[] = [];
	for (let x = minX; x <= maxX; x += (maxX - minX) / resolution) {
		const y = gaussian(x);
		points.push({ x: x, y: y });
	}

	function toPath(points: Point[], width: number, height: number) {
		// Left side of the Gaussian curve
		const leftPath = points
			.map((p, i) => {
				const px = ((p.y - minY) / (maxY - minY)) * (width / 2); // Scale y to half width (left side)
				const py = ((p.x - minX) / (maxX - minX)) * height; // Scale x to full height
				//TODO: Fix hardcorded 50
				return `${i === 0 ? "M" : "L"} ${50 - px} ${height - py}`; // Flip y-axis
			})
			.join(" ");

		// Right side of the Gaussian curve
		const rightPath = points
			.map((p, i) => {
				const px = ((p.y - minY) / (maxY - minY)) * (width / 2); // Scale y to half width (right side)
				const py = ((p.x - minX) / (maxX - minX)) * height; // Scale x to full height
				//TODO: Fix hardcorded 50
				return `${i === 0 ? "M" : "L"} ${50 + px} ${height - py}`; // Flip y-axis
			})
			.join(" ");

		return leftPath + " " + rightPath;
	}

	function toSVGCoords(x: number, y: number, width: number, height: number) {
		const gaussX = minX + (maxX - minX) * y;
		const gaussY = gaussian(gaussX);

		const py = height - y * height;

		const pgaussY = -gaussY + gaussY * 2 * x;
		const px = 50 - ((pgaussY - minY) / (maxY - minY)) * (width / 2);

		return { x: px, y: py };
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
			d={toPath(points, 100, 100)}
			stroke="red"
			fill="none"
			stroke-width="3"
			vector-effect="non-scaling-stroke"
		/>
		{#each pointData as p}
			<circle
				cx={toSVGCoords(p.x, p.y, 100, 100).x}
				cy={toSVGCoords(p.x, p.y, 100, 100).y}
				r="1"
				class="node"
			/>
		{/each}
	</svg>
</div>

<style>
	.node {
		fill: blue;
	}
</style>
