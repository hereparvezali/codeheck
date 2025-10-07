// LocalStorage utility functions for auth data

const USER_STORAGE_KEY = "codeheck_user";
const TOKEN_STORAGE_KEY = "codeheck_access_token";

export interface StoredUser {
    id: number;
    username: string;
    email: string;
    access_token: string;
}

/**
 * Save user data to localStorage
 */
export const saveUser = (user: StoredUser): void => {
    try {
        localStorage.setItem(USER_STORAGE_KEY, JSON.stringify(user));
        localStorage.setItem(TOKEN_STORAGE_KEY, user.access_token);
    } catch (e) {
        console.error("Error saving user to localStorage:", e);
    }
};

/**
 * Get user data from localStorage
 */
export const getUser = (): StoredUser | null => {
    try {
        const savedUser = localStorage.getItem(USER_STORAGE_KEY);
        if (savedUser) {
            return JSON.parse(savedUser);
        }
    } catch (e) {
        console.error("Error loading user from localStorage:", e);
        // Clear corrupted data
        clearUser();
    }
    return null;
};

/**
 * Get access token from localStorage
 */
export const getToken = (): string | null => {
    try {
        return localStorage.getItem(TOKEN_STORAGE_KEY);
    } catch (e) {
        console.error("Error loading token from localStorage:", e);
        return null;
    }
};

/**
 * Save access token to localStorage
 */
export const saveToken = (token: string): void => {
    try {
        localStorage.setItem(TOKEN_STORAGE_KEY, token);
    } catch (e) {
        console.error("Error saving token to localStorage:", e);
    }
};

/**
 * Clear all auth data from localStorage
 */
export const clearUser = (): void => {
    try {
        localStorage.removeItem(USER_STORAGE_KEY);
        localStorage.removeItem(TOKEN_STORAGE_KEY);
    } catch (e) {
        console.error("Error clearing user from localStorage:", e);
    }
};

/**
 * Check if user data exists in localStorage
 */
export const hasStoredAuth = (): boolean => {
    return !!(localStorage.getItem(USER_STORAGE_KEY) && localStorage.getItem(TOKEN_STORAGE_KEY));
};

/**
 * Update user data in localStorage (for partial updates)
 */
export const updateUser = (updates: Partial<StoredUser>): void => {
    const currentUser = getUser();
    if (currentUser) {
        const updatedUser = { ...currentUser, ...updates };
        saveUser(updatedUser);
    }
};