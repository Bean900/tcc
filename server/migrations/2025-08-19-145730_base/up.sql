-- ========================================
-- Address
-- ========================================
CREATE TABLE "Address" (
    "id" UUID PRIMARY KEY,
    "address" TEXT NOT NULL,
    "latitude" DOUBLE PRECISION NOT NULL,
    "longitude" DOUBLE PRECISION NOT NULL
);

-- ========================================
-- Plan
-- ========================================
CREATE TABLE "Plan" (
    "id" UUID PRIMARY KEY,
    "access" JSONB NOT NULL,
    "introduction" TEXT NULL,
    "walking_paths" JSONB NOT NULL
);

-- ========================================
-- Share Config
-- ========================================
CREATE TYPE team_fields as enum(
    'mail','phone','members','diets'
);

CREATE TABLE "Share" (
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
CREATE TABLE "CookAndRun" (
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
        REFERENCES "Plan" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_cookandrun_course_sp FOREIGN KEY ("start_point") 
        REFERENCES "Address" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_cookandrun_course_ep FOREIGN KEY ("end_point") 
        REFERENCES "Address" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_cookandrun_course_share FOREIGN KEY ("share_team_config") 
        REFERENCES "Share" ("id") ON DELETE CASCADE
);

CREATE INDEX idx_cookandrun_user_id ON "CookAndRun" ("user_id");

-- ========================================
-- Team
-- ========================================
CREATE TABLE "Team" (
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
    FOREIGN KEY ("cook_and_run_id") REFERENCES "CookAndRun" ("id"),
    FOREIGN KEY ("address") REFERENCES "Address" ("id") ON DELETE CASCADE
);

CREATE INDEX idx_team_cook_and_run ON "Team" ("cook_and_run_id");
CREATE INDEX idx_team_user ON "Team" ("created_by_user");

-- ========================================
-- Note
-- ========================================
CREATE TABLE "Note" (
    "id" UUID PRIMARY KEY,
    "team_id" UUID NOT NULL,
    "headline" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "created" TIMESTAMPTZ NOT NULL,
    FOREIGN KEY ("team_id") REFERENCES "Team"("id") ON DELETE CASCADE
);

CREATE INDEX idx_note_team_id ON "Note" ("team_id");

-- ========================================
-- Course
-- ========================================
CREATE TABLE "Course" (
    "id" UUID PRIMARY KEY,
    "cook_and_run_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "time" TEXT NOT NULL,
    FOREIGN KEY ("cook_and_run_id") REFERENCES "CookAndRun" ("id")
);

CREATE INDEX idx_course_cook_and_run ON "Course" ("cook_and_run_id");

ALTER TABLE "CookAndRun" ADD CONSTRAINT fk_cookandrun_course_wmh FOREIGN KEY ("course_with_multiple_hosts") REFERENCES "Course" ("id") ON DELETE CASCADE;

-- ========================================
-- Hosting
-- ========================================
CREATE TABLE "Hosting" (
    "id" UUID PRIMARY KEY,
    "plan_id" UUID NOT NULL,
    "course_id" UUID NOT NULL,
    "team_id" UUID NOT NULL,
    "guest_team_ids" JSONB NOT NULL,
    CONSTRAINT fk_hosting_plan FOREIGN KEY ("plan_id") 
        REFERENCES "Plan" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_hosting_course FOREIGN KEY ("course_id") 
        REFERENCES "Course" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_hosting_team FOREIGN KEY ("team_id") 
        REFERENCES "Team" ("id") ON DELETE CASCADE
);

CREATE INDEX idx_hosting_course_id ON "Hosting" ("course_id");
CREATE INDEX idx_hosting_team_id ON "Hosting" ("team_id");
CREATE INDEX idx_hosting_plan_id ON "Hosting" ("plan_id");