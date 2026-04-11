import { useGeolocation } from "@vueuse/core";
import type { Location } from "@/types";
import { computed } from "vue";

async function fetchStateName(lat: number, lon: number): Promise<string | null> {
    try {
        if (!isFinite(lat) || !isFinite(lon)) {
            console.warn("Invalid coordinates provided to fetchStateName:", { lat, lon });
            return null;
        }
        const res = await fetch(
            `https://nominatim.openstreetmap.org/reverse?lat=${lat}&lon=${lon}&format=json`,
            { headers: { 'User-Agent': 'VueApp-Location-Service' } }
        );
        if (!res.ok) return null;
        const data = await res.json();

        console.log(data.address);
        return data.address?.state ?? null;
    }
    catch (err: any) {
        console.error("Nominatim Error:", err);
        return null;
    }
}


export default {
    async getLocation(): Promise<Location> {
        return new Promise((resolve, reject) => {
            if (!navigator.geolocation) {
                reject(new Error("Geolocation not supported."))
                return;
            }

            navigator.geolocation.getCurrentPosition(
                async (position) => {
                    const { longitude, latitude } = position.coords;
                    const stateName = await fetchStateName(latitude, longitude) ?? 'unknown';

                resolve({
                    latitude,
                    longitude,
                    stateName
                })
        },
            (err) => {
                console.error("GPS Error: ", err);
                resolve({ latitude: 0, longitude: 0, stateName: 'unknown'});
            }, { timeout: 10000 }
        );
    });
}
};