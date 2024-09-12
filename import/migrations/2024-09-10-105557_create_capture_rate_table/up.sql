CREATE TABLE
    capture_rate AS
WITH
    tmp AS (
        SELECT DISTINCT
            capture_rate AS `value`
        FROM
            opendata
        ORDER BY
            `value`
    )
SELECT
    UUID () AS `capture_rate_id`,
    `value`
FROM
    tmp;

ALTER TABLE capture_rate MODIFY COLUMN `capture_rate_id` CHAR(36) PRIMARY KEY,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
