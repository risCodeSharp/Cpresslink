import type { DeviceInformation } from "@/types";

export default {

    getDeviceCategory(): 'mobile' | 'tablet' | 'laptop' | 'desktop' | 'tv' {
        const ua = navigator.userAgent;
        const width = window.innerWidth;
        const hasTouch = navigator.maxTouchPoints > 0;

        // TV Detection
        // TVs usually identify themselves in the User Agent
        const isTV = /TV|SmartTV|GoogleTV|AppleTV|HbbTV|CastTV|Plex|Roku|Viera|BRAVIA/i.test(ua);
        if (isTV) return 'tv';

        // Tablet Detection
        const isTabletUA = /(tablet|ipad|playbook|silk)|(android(?!.*mobi))/i.test(ua);
        const isIPadOS = (navigator.platform === 'MacIntel' && hasTouch);
        if (isTabletUA || isIPadOS || (hasTouch && width >= 768 && width <= 1280)) {
            return 'tablet';
        }

        // Mobile Detection
        const isMobileUA = /Mobile|iP(hone|od)|Android|BlackBerry|IEMobile|Opera M(obi|ini)/.test(ua);
        if (isMobileUA || (hasTouch && width < 768)) {
            return 'mobile';
        }

        // Laptop vs. Desktop Detection
        // We check for hardware features common in laptops
        const hasBattery = 'getBattery' in navigator;

        // High-density screens (Retina) + Touch are very common on laptops
        const isHighResTouch = hasTouch && window.devicePixelRatio > 1;

        if (hasBattery || isHighResTouch) {
            return 'laptop';
        }
        // Default to Desktop
        return 'desktop';
    },



    getBrowserAndDeviceInfo(): DeviceInformation {
        const ua = navigator.userAgent;
        const width = window.innerWidth;
        const hasTouch = 'ontouchstart' in window || navigator.maxTouchPoints > 0;

        let browserName = "Unknown";
        let deviceType = "Unknown";
        let deviceName = "Unknown Device";
        let os = "Unknown OS";

        // 1. Detect Browser
        if (/chrome|crios|crmo/i.test(ua) && !/edge|edg/i.test(ua) && !/opr\//i.test(ua)) {
            browserName = "Google Chrome";
        } else if (/firefox|fxios/i.test(ua)) {
            browserName = "Mozilla Firefox";
        } else if (/safari/i.test(ua) && !/chrome|crios|crmo/i.test(ua)) {
            browserName = "Safari";
        } else if (/edg/i.test(ua)) {
            browserName = "Microsoft Edge";
        } else if (/opr\//i.test(ua)) {
            browserName = "Opera";
        } else if (/msie|trident/i.test(ua)) {
            browserName = "Internet Explorer";
        }

        // 2. Detect OS
        if (/windows nt 10/i.test(ua)) os = "Windows 10/11";
        else if (/windows nt 6\.3/i.test(ua)) os = "Windows 8.1";
        else if (/windows nt 6\.2/i.test(ua)) os = "Windows 8";
        else if (/windows nt 6\.1/i.test(ua)) os = "Windows 7";
        else if (/macintosh|mac os x/i.test(ua)) os = "macOS";
        else if (/android/i.test(ua)) os = "Android";
        else if (/iphone|ipad|ipod/i.test(ua)) os = "iOS";
        else if (/linux/i.test(ua)) os = "Linux";

        // 3. Detect Device Category (Mobile, Tablet, Laptop, TV, Desktop)
        if (/TV|SmartTV|GoogleTV|AppleTV|Roku|CastTV/i.test(ua)) {
            deviceType = "TV";
        } else if (/(tablet|ipad|playbook|silk)|(android(?!.*mobi))/i.test(ua) || (os === 'macOS' && hasTouch)) {
            deviceType = "Tablet";
        } else if (/mobile|iphone|ipod|android.*mobi/i.test(ua)) {
            deviceType = "Mobile";
        } else {
            // If it's a large screen and has a battery or touch, it's likely a Laptop
            const hasBattery = 'getBattery' in navigator;
            deviceType = (hasTouch || hasBattery) ? "Laptop" : "Desktop";
        }

        // 4. Detect Specific Device Name (Marketing Name)
        if (os === "iOS") {
            if (/iphone/i.test(ua)) deviceName = "iPhone";
            else if (/ipad/i.test(ua)) deviceName = "iPad";
            else if (/ipod/i.test(ua)) deviceName = "iPod";
        } else if (os === "Android") {
            const match = ua.match(/\(([^;]+);([^;]+);/);
            // Ensure match exists and has the expected group before accessing index [2]
            deviceName = (match && match[2]) ? match[2].trim() : "Android Device";
        } else if (os === "macOS") {
            deviceName = "Macintosh";
        } else if (os.includes("Windows")) {
            deviceName = "Windows PC";
        } else if (deviceType === "TV") {
            deviceName = "Smart TV";
        }

        return {
            browser: browserName,
            device: deviceType,
            deviceName: deviceName,
            os: os,
            userAgent: ua
        };

    }
}