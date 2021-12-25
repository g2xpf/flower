import type { API, APIKey } from './api';

declare global {
  interface Window extends Record<APIKey, API> {}
}
