CREATE TABLE
    generation AS
WITH
    tmp AS (
        SELECT DISTINCT
            generation AS `value`
        FROM
            opendata
        ORDER BY
            `value`
    )
SELECT
    UUID () AS `generation_id`,
    `value`
FROM
    tmp;

ALTER TABLE generation MODIFY COLUMN `generation_id` CHAR(36) PRIMARY KEY,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
