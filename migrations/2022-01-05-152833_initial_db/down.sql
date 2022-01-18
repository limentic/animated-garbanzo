-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS customers, addresses, housings, folders, appointements, projects CASCADE;
DROP TYPE role;
