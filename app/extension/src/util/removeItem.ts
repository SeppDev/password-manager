export default function removeItem<T>(arr: Array<T>, index: number): Array<T> {
  if (index < 0 || index >= arr.length) {
    console.error("Index out of bounds");
    return arr; // Return the original array if the index is invalid
  }
  return [...arr.slice(0, index), ...arr.slice(index + 1)];
}
