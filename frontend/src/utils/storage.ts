

const USER_STORAGE_KEY = "codeheck_user";
const TOKEN_STORAGE_KEY = "codeheck_access_token";

export interface StoredUser {
    id: number;
    username: string;
    email: string;
    access_token: string;
}


export const saveUser = (user: StoredUser): void => {
    try {
        localStorage.setItem(USER_STORAGE_KEY, JSON.stringify(user));
        localStorage.setItem(TOKEN_STORAGE_KEY, user.access_token);
    } catch (e) {
        console.error("Error saving user to localStorage:", e);
    }
};


export const getUser = (): StoredUser | null => {
    try {
        const savedUser = localStorage.getItem(USER_STORAGE_KEY);
        if (savedUser) {
            return JSON.parse(savedUser);
        }
    } catch (e) {
        console.error("Error loading user from localStorage:", e);

        clearUser();
    }
    return null;
};


export const getToken = (): string | null => {
    try {
        return localStorage.getItem(TOKEN_STORAGE_KEY);
    } catch (e) {
        console.error("Error loading token from localStorage:", e);
        return null;
    }
};


export const saveToken = (token: string): void => {
    try {
        localStorage.setItem(TOKEN_STORAGE_KEY, token);
    } catch (e) {
        console.error("Error saving token to localStorage:", e);
    }
};


export const clearUser = (): void => {
    try {
        localStorage.removeItem(USER_STORAGE_KEY);
        localStorage.removeItem(TOKEN_STORAGE_KEY);
    } catch (e) {
        console.error("Error clearing user from localStorage:", e);
    }
};


export const hasStoredAuth = (): boolean => {
    return !!(localStorage.getItem(USER_STORAGE_KEY) && localStorage.getItem(TOKEN_STORAGE_KEY));
};


export const updateUser = (updates: Partial<StoredUser>): void => {
    const currentUser = getUser();
    if (currentUser) {
        const updatedUser = { ...currentUser, ...updates };
        saveUser(updatedUser);
    }
};
