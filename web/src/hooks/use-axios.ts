"use client";

import { useEffect } from "react";
import { useAuth } from "../context/AuthContext";
// import { useRouter } from "next/navigation";
// import { IAuthResponse } from "@/features/auth/auth.types";
import { api } from "@/lib/api";
import { useNavigate } from "@tanstack/react-router";
import type { IAuthResponse } from "@/features/auth/auth.types";

export const useAxios = () => {
  const { accessToken, setAccessToken } = useAuth();
  const navigate = useNavigate();
  useEffect(() => {
    // Request interceptor
    const requestIntercept = api.interceptors.request.use((config) => {
      if (accessToken) {
        config.headers.Authorization = `Bearer ${accessToken}`;
      }
      return config;
    },
      err => Promise.reject(err)
    );

    // Response interceptor
    const responseIntercept = api.interceptors.response.use(
      (response) => response,
      async (error) => {
        const originalRequest = error.config;
        
        // Prevent infinite loop: if refresh endpoint fails, don't retry
        if (originalRequest?.url?.includes("/auth/refresh")) {
          setAccessToken(""); // logout
          navigate({ to: '/login' });
          return Promise.reject(error);
        }
        
        if (error.response?.status === 401 && !originalRequest._retry) {
          originalRequest._retry = true;
          try {
            const res = await api.get<IAuthResponse>("/auth/refresh"); // refresh token endpoint
            setAccessToken(res.data.access_token);
            originalRequest.headers.Authorization = `Bearer ${res.data.access_token}`;
            return api(originalRequest);
          } catch (err) {
            setAccessToken(""); // logout
            navigate({ to: '/login' });
            return Promise.reject(err);
          }
        }
        return Promise.reject(error);
      }
    );

    return () => {
      api.interceptors.request.eject(requestIntercept);
      api.interceptors.response.eject(responseIntercept);
    };
  }, [accessToken, setAccessToken]);

  return api;
};
