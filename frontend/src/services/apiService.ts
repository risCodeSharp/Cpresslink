import axios, { AxiosHeaders } from "axios";

const api = axios.create({
  baseURL: "http://localhost:8030",
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
});

api.interceptors.request.use((config) => {
  const token = localStorage.getItem("accessToken");
  console.log("➡️ REQUEST:", {
    url: config.url,
    method: config.method,
    data: config.data,
    headers: config.headers,
  });

  if (token && config.headers) {
    config.headers.set("Authorization", `Bearer ${token}`);
  }

  return config;
});


api.interceptors.request.use((config) => {
  const token = localStorage.getItem("accessToken");

  console.log("➡️ REQUEST:", {
    url: config.url,
    method: config.method,
    data: config.data,
    headers: config.headers,
  });

  if (token) {
    if (!config.headers) {
      config.headers = {} as any;
    }
    config.headers["Authorization"] = `Bearer ${token}`;
  }

  return config;
});

export default api;