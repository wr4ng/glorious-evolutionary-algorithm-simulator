export function downloadCSV(content: string) {
	const blob = new Blob([content], {
		type: "text/csv;charset=utf-8;",
	});
	const url = URL.createObjectURL(blob);
	const link = document.createElement("a");

	link.setAttribute("href", url);
	link.setAttribute("download", "data.csv");

	link.click();
	URL.revokeObjectURL(url);
}
