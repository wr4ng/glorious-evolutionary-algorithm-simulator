const algorithmNameMap: { [id: string]: string } = {
	"OnePlusOneEA": "(1+1) EA",
	"SimulatedAnnealing": "SA",
	"ACO": "ACO",
};

export function mapAlgorithmName(problem: string): string {
	if (problem in algorithmNameMap) {
		return algorithmNameMap[problem];
	}
	return problem;
}
