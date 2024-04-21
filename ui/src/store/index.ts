import Cookies from 'js-cookie';
import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';

export interface AppState {
  isLoggedIn: boolean;
  profile: any;
  projects: any[];
  setProjects: (projects: any[]) => void;
  setProfile: (profile: any) => void;
  login: () => void;
  logout: () => void;
}

export const useAppState = create<AppState>()(
  devtools(
    persist(
      (set) => ({
        isLoggedIn: false,
        profile: null,
        projects: [],
        setProjects: (projects) => set({ projects }),
        setProfile: (profile) => set({ profile }),
        login: async () => {
          set({ isLoggedIn: true });
        },
        logout: async () => {
          Cookies.remove('token');
          set({ isLoggedIn: false });
        },
      }),
      {
        name: 'app-storage',
      }
    )
  )
);
