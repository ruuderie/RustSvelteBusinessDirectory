const API_URL = import.meta.env.VITE_API_URL;

console.log("API_URL:", API_URL);

export async function fetchBusinesses() {
  console.log("Fetching businesses from:", `${API_URL}/businesses`);
  const response = await fetch(`${API_URL}/businesses`);
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to fetch businesses");
  }
  return response.json();
}

export async function searchBusinesses(query) {
  console.log("Searching businesses with query:", query);
  const response = await fetch(
    `${API_URL}/businesses/search?q=${encodeURIComponent(query)}`,
  );
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to search businesses");
  }
  return response.json();
}

export async function fetchBusinessById(id) {
  console.log("Fetching business details for id:", id);
  const response = await fetch(`${API_URL}/businesses/${id}`);
  console.log("Response:", response);
  if (!response.ok) {
    throw new Error("Failed to fetch business details");
  }
  return response.json();
}
