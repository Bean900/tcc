-- ========================================
-- Address
-- ========================================
CREATE TABLE "address" (
    "id" UUID PRIMARY KEY,
    "address_text" TEXT NOT NULL,
    "latitude" DOUBLE PRECISION NOT NULL,
    "longitude" DOUBLE PRECISION NOT NULL
);

-- ========================================
-- Plan
-- ========================================
CREATE TYPE access as enum(
    'link','account' 
);

CREATE TABLE "plan" (
    "id" UUID PRIMARY KEY,
    "access" access[] NULL,
    "introduction" TEXT NULL,
    "walking_paths" JSONB NOT NULL
);

-- ========================================
-- Share Config
-- ========================================
CREATE TYPE team_fields as enum(
    'mail','phone','members','diets'
);

CREATE TABLE "share" (
    "id" UUID PRIMARY KEY,
    "created" TIMESTAMPTZ NOT NULL,
    "invite_text" TEXT NOT NULL,
    "needs_login" BOOLEAN NOT NULL,
    "default_needs_check" BOOLEAN NOT NULL,
    "required_fields" team_fields[] NULL,
    "max_teams" INTEGER NULL,
    "registration_deadline" TIMESTAMPTZ NULL
);

-- ========================================
-- CookAndRun
-- ========================================
CREATE TABLE "cook_and_run" (
    "id" UUID PRIMARY KEY,
    "user_id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "created" TIMESTAMPTZ NOT NULL,
    "edited" TIMESTAMPTZ NOT NULL,
    "occur" TIMESTAMPTZ NOT NULL,
    "course_with_multiple_hosts" UUID NULL,
    "start_point" UUID NULL,
    "end_point" UUID NULL,
    "share_team_config" UUID NULL,
    "plan" UUID NULL,
    CONSTRAINT fk_cookandrun_plan FOREIGN KEY ("plan") 
        REFERENCES "plan" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_cookandrun_course_sp FOREIGN KEY ("start_point") 
        REFERENCES "address" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_cookandrun_course_ep FOREIGN KEY ("end_point") 
        REFERENCES "address" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_cookandrun_course_share FOREIGN KEY ("share_team_config") 
        REFERENCES "share" ("id") ON DELETE CASCADE
);

CREATE INDEX idx_cookandrun_user_id ON "cook_and_run" ("user_id");

-- ========================================
-- Team
-- ========================================
CREATE TABLE "team" (
    "id" UUID PRIMARY KEY,
    "cook_and_run_id" UUID NOT NULL,
    "created_by_user" TEXT NULL,
    "name" TEXT NOT NULL,
    "created" TIMESTAMPTZ NOT NULL,
    "edited" TIMESTAMPTZ NOT NULL,
    "address" UUID NOT NULL,
    "mail" TEXT NULL,
    "phone" TEXT NULL,
    "members" INTEGER NULL,
    "diets" TEXT NULL,
    "needs_check" BOOLEAN NOT NULL,
    FOREIGN KEY ("cook_and_run_id") REFERENCES "cook_and_run" ("id"),
    FOREIGN KEY ("address") REFERENCES "address" ("id") ON DELETE CASCADE
);

CREATE INDEX idx_team_cook_and_run ON "team" ("cook_and_run_id");
CREATE INDEX idx_team_user ON "team" ("created_by_user");

-- ========================================
-- Note
-- ========================================
CREATE TABLE "note" (
    "id" UUID PRIMARY KEY,
    "team_id" UUID NOT NULL,
    "headline" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "created" TIMESTAMPTZ NOT NULL,
    FOREIGN KEY ("team_id") REFERENCES "team"("id") ON DELETE CASCADE
);

CREATE INDEX idx_note_team_id ON "note" ("team_id");

-- ========================================
-- Course
-- ========================================
CREATE TABLE "course" (
    "id" UUID PRIMARY KEY,
    "cook_and_run_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "time" TEXT NOT NULL,
    FOREIGN KEY ("cook_and_run_id") REFERENCES "cook_and_run" ("id")
);

CREATE INDEX idx_course_cook_and_run ON "course" ("cook_and_run_id");

ALTER TABLE "cook_and_run" ADD CONSTRAINT fk_cookandrun_course_wmh FOREIGN KEY ("course_with_multiple_hosts") REFERENCES "course" ("id") ON DELETE CASCADE;

-- ========================================
-- Hosting
-- ========================================
CREATE TABLE "hosting" (
    "id" UUID PRIMARY KEY,
    "plan_id" UUID NOT NULL,
    "course_id" UUID NOT NULL,
    "team_id" UUID NOT NULL,
    "guest_team_ids" JSONB NOT NULL,
    CONSTRAINT fk_hosting_plan FOREIGN KEY ("plan_id") 
        REFERENCES "plan" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_hosting_course FOREIGN KEY ("course_id") 
        REFERENCES "course" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_hosting_team FOREIGN KEY ("team_id") 
        REFERENCES "team" ("id") ON DELETE CASCADE
);

CREATE INDEX idx_hosting_course_id ON "hosting" ("course_id");
CREATE INDEX idx_hosting_team_id ON "hosting" ("team_id");
CREATE INDEX idx_hosting_plan_id ON "hosting" ("plan_id");