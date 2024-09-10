const API_URL = 'https://5c25f946-99a5-403b-9a0e-20f0b87e3036-8000.riker.replit.dev/api';

export async function fetchBusinesses() {
  const response = await fetch(`${API_URL}/businesses`);
  if (!response.ok) {
    throw new Error('Failed to fetch businesses');
  }
  return response.json();
}

export async function searchBusinesses(query) {
  const response = await fetch(`${API_URL}/businesses/search?q=${encodeURIComponent(query)}`);
  if (!response.ok) {
    throw new Error('Failed to search businesses');
  }
  return response.json();
}

export async function fetchBusinessById(id) {
  const response = await fetch(`${API_URL}/businesses/${id}`);
  if (!response.ok) {
    throw new Error('Failed to fetch business details');
  }
  return response.json();
}
