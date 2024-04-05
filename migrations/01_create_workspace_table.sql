CREATE TABLE
    IF NOT EXISTS "user" (
        user_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        user_name TEXT NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS workspace_status_master (
        workspace_status_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        status_name TEXT NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS workspace (
        workspace_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
        workspace_name TEXT NOT NULL,
        description TEXT,
        workspace_status_id INTEGER NOT NULL REFERENCES workspace_status_master,
        user_id INTEGER NOT NULL REFERENCES "user",
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );