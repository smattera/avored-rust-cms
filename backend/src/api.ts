import axios from "axios";

// const injectAccessToken = (config: InternalAxiosRequestConfig) => {
//   const accessToken = localStorage.getItem("access_token");
//   if (accessToken)
//     config.headers!.common["Authorization"] = `Bearer ${accessToken}`;
//   return config;
// };


// const accessToken = localStorage.getItem("access_token");
  
const config = {
  baseURL: "http://localhost:8080",
  headers: {
      // accessToken ? 'Authoziation': 'Bearer ' + accessToken : null
  }
};

const API = axios.create(config);

// API.interceptors.request.use(injectAccessToken);

export default API;