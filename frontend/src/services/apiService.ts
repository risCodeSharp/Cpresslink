import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:8030",
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
});

// ---------------------
// REQUEST INTERCEPTOR
// ---------------------
api.interceptors.request.use((config) => {
  const token = localStorage.getItem("accessToken"); // FIXED KEY CONSISTENCY

  console.log("➡️ REQUEST:", {
    url: config.url,
    method: config.method,
    data: config.data,
    headers: config.headers,
  });

  if (token) {
    config.headers = config.headers ?? {};
    config.headers.Authorization = `Bearer ${token}`;
  }

  return config;
});

// ---------------------
// RESPONSE INTERCEPTOR
// ---------------------
api.interceptors.response.use(
  (response) => {
    console.log("⬅️ RESPONSE:", {
      url: response.config.url,
      status: response.status,
      data: response.data,
    });

    return response;
  },
  (error) => {
    console.log("❌ RESPONSE ERROR:", {
      message: error.message,
      status: error.response?.status,
      data: error.response?.data,
      config: error.config,
    });

    // handle unauthorized
    if (error.response?.status === 401) {
      localStorage.removeItem("accessToken");

      if (window.location.hash !== "#/login") {
        window.location.hash = "#/login";
      }
    }

    return Promise.reject(error);
  }
);

export default api;