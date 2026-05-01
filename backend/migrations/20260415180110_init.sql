-- Add migration script here
CREATE TABLE users(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    email VARCHAR(100) NOT NULL,
    username VARCHAR(50) NOT NULL,
    oauth_provider VARCHAR(50),
    oauth_id TEXT,
    password_hash TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Ensure either oauth or password login is used
    CHECK (
        (oauth_provider IS NOT NULL AND oauth_id IS NOT NULL)
        OR (password_hash IS NOT NULL)
    ),
    UNIQUE(email, username, oauth_provider),
    UNIQUE (oauth_provider, oauth_id)
);



CREATE TABLE shortlinks (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    short_code  VARCHAR(20) NOT NULL,

    destination TEXT NOT NULL,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ DEFAULT (NOW() + interval '1 year'),
    notified BOOLEAN DEFAULT FALSE,

    UNIQUE (user_id, slug),
    UNIQUE (user_id, short_code)
);

CREATE TABLE clicks (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    shortlink_id BIGINT NOT NULL REFERENCES shortlinks(id) ON DELETE CASCADE,

    referrer TEXT,
    clicked_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE click_locations (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    click_id BIGINT NOT NULL REFERENCES clicks(id) ON DELETE CASCADE,
    
    country VARCHAR(100),
    region VARCHAR(100),
    city VARCHAR(100)
);

CREATE TABLE click_user_agents (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    click_id BIGINT NOT NULL REFERENCES clicks(id) ON DELETE CASCADE,
    
    browser VARCHAR(100),
    os VARCHAR(100),
    device VARCHAR(100),
    is_bot BOOLEAN DEFAULT FALSE,
    bot_name VARCHAR(100)
);

-- CREATE TABLE link_health_checks (
--     id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
--     shortlink_id BIGINT NOT NULL REFERENCES shortlinks(id) ON DELETE CASCADE,

--     status_code BIGINT,   -- HTTP status (200, 404, 500)
--     is_healthy BOOLEAN,   -- derived from status code
--     latency_ms BIGINT        -- response time of destination URL
-- );

CREATE TABLE system_health_checks (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    db_ok BOOLEAN,
    db_latency_ms BIGINT,
    
    redis_ok BOOLEAN,
    redis_latency_ms BIGINT,

    checked_at TIMESTAMPTZ DEFAULT NOW()
);
