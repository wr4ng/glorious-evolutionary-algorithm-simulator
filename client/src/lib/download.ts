export function downloadCSV(content: string, filename: string) {
	const blob = new Blob([content], {
		type: "text/csv;charset=utf-8;",
	});
	const url = URL.createObjectURL(blob);
	const link = document.createElement("a");

	link.setAttribute("href", url);
	link.setAttribute("download", filename);

	link.click();
	URL.revokeObjectURL(url);
}
