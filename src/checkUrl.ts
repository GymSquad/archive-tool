import axios from "axios";
import { CHECK_URL_TIMEOUT } from "./config";

export async function checkUrl(url: string) {
  console.log(`Checking ${url}...`);

  try {
    await axios.get(url, {
      timeout: CHECK_URL_TIMEOUT,
      signal: AbortSignal.timeout(CHECK_URL_TIMEOUT),
    });
    return true;
  } catch (err) {
    return false;
  }
}
