export function toUpperCaseFirst(label: string) {
	if (label.length === 0) {
		return label;
	}
	const firstChar = label.charAt(0).toUpperCase();
	return firstChar + label.slice(1);
}
