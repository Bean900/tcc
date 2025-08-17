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
-- Team
-- ========================================
CREATE TABLE "Team" (
    "id" UUID PRIMARY KEY,
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
    "note_list" JSONB NOT NULL,
    CONSTRAINT fk_team_address FOREIGN KEY ("address") 
        REFERENCES "Address" ("id") ON DELETE CASCADE
);

CREATE INDEX idx_team_address ON "Team" ("address");
CREATE INDEX idx_team_name ON "Team" ("name");

-- ========================================
-- Note
-- ========================================
CREATE TABLE "Note" (
    "id" UUID PRIMARY KEY,
    "headline" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "created" TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_note_created ON "Note" ("created");

-- ========================================
-- Course
-- ========================================
CREATE TABLE "Course" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL,
    "time" TEXT NOT NULL
);

CREATE INDEX idx_course_name ON "Course" ("name");

-- ========================================
-- Hosting
-- ========================================
CREATE TABLE "Hosting" (
    "id" UUID PRIMARY KEY,
    "course_id" UUID NOT NULL,
    "team_id" UUID NOT NULL,
    "guest_team_ids" JSONB NOT NULL,
    CONSTRAINT fk_hosting_course FOREIGN KEY ("course_id") 
        REFERENCES "Course" ("id") ON DELETE CASCADE,
    CONSTRAINT fk_hosting_team FOREIGN KEY ("team_id") 
        REFERENCES "Team" ("id") ON DELETE CASCADE
);

CREATE INDEX idx_hosting_course_id ON "Hosting" ("course_id");
CREATE INDEX idx_hosting_team_id ON "Hosting" ("team_id");

-- ========================================
-- Plan
-- ========================================
CREATE TABLE "Plan" (
    "id" UUID PRIMARY KEY,
    "access" JSONB NOT NULL,
    "introduction" TEXT NULL,
    "hosting_assignments" JSONB NOT NULL,
    "walking_paths" JSONB NOT NULL
);

-- ========================================
-- CookAndRun
-- ========================================
CREATE TABLE "CookAndRun" (
    "id" UUID PRIMARY KEY,
    "name" TEXT NOT NULL,
    "created" TIMESTAMPTZ NOT NULL,
    "edited" TIMESTAMPTZ NOT NULL,
    "occur" TIMESTAMPTZ NOT NULL,
    "team_list" JSONB NOT NULL,
    "course_list" JSONB NOT NULL,
    "course_with_multiple_hosts" JSONB NULL,
    "start_point" TEXT NULL,
    "end_point" TEXT NULL,
    "share_team_config" TEXT NULL,
    "plan" UUID NULL,
    CONSTRAINT fk_cookandrun_plan FOREIGN KEY ("plan") 
        REFERENCES "Plan" ("id") ON DELETE SET NULL
);

CREATE INDEX idx_cookandrun_name ON "CookAndRun" ("name");
CREATE INDEX idx_cookandrun_occur ON "CookAndRun" ("occur");
