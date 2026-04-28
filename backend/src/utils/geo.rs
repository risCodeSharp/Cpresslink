use maxminddb::{MaxMindDbError, Reader, geoip2};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug)]
pub struct GeoInfo {
    pub ip: String,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct RequestGeoLocation {
    ip: String,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    loc: Option<String>,
    org: Option<String>,
    postal: Option<String>,
}

pub type GeoDB = Reader<Vec<u8>>;

pub fn init_geo_db() -> GeoDB {
    Reader::open_readfile("data/GeoLite2-City.mmdb").expect("Failed to load the geo .mmdb file")
}

async fn lookup_online(ip: &str) -> Option<GeoInfo> {
    let url = format!("https://ipinfo.io/{}/json", ip.trim());

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(location) = response.json::<RequestGeoLocation>().await {
            return Some(GeoInfo {
                ip: ip.to_string(),
                country: location.country,
                region: location.region,
                city: location.city,
            });
        }
    }
    None
}

pub async fn lookup(db: &GeoDB, ip: &str) -> Result<GeoInfo, MaxMindDbError> {
    let ip_addr: IpAddr = ip.parse().map_err(|e| {
        MaxMindDbError::from(MaxMindDbError::decoding(
            format!("failed to parse ip address. [Error]: {e:?}"),
        ))
    })?;

    // set the both to none
    let mut region: Option<String> = None;
    let mut city: Option<String> = None;
    
    // lookup the location
    let result = db.lookup(ip_addr)?;

    if let Some(geo_city) = result.decode::<geoip2::City>()? {
        // if any city named in english language is found then set to city 
        if let Some(name) = geo_city.city.names.english {
            city = Some(name.to_string());
        }
        // if any region named in english language is found then set to region
        for sub in &geo_city.subdivisions {
            if let Some(name) = sub.names.english {
                region = Some(name.to_string());
            }
        }
    }
    
    // set country to none
    let mut country: Option<String> = None;
    
    
    if let Some(name) = db.lookup(ip_addr)?.decode::<geoip2::Country>()? {
        // if any country named in english language is found then set to country
        if let Some(name) = name.country.names.english {
            country = Some(name.to_string());
        }
    }

    // fallback to api request for the none field here 
    if region.is_none() || country.is_none() || city.is_none() {
        // retrieve the response of api request 
        if let Some(info) = lookup_online(&ip).await {
            // set variable to use data where it is none
            country = country.or(info.country);
            city = city.or(info.city);
            region = region.or(info.region);
        }
    }

    // compose all into a GeoInfo Struct and return it
    let info = GeoInfo {
        ip: ip.to_string(),
        country,
        region,
        city,
    };
    Ok(info)
}
