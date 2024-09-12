CREATE TABLE
    experience_growth AS
WITH
    tmp AS (
        SELECT DISTINCT
            experience_growth AS `value`
        FROM
            opendata
        ORDER BY
            `value`
    )
SELECT
    UUID () AS `experience_growth_id`,
    `value`
FROM
    tmp;

ALTER TABLE experience_growth MODIFY COLUMN `experience_growth_id` CHAR(36) PRIMARY KEY,
MODIFY COLUMN `value` INT UNSIGNED NOT NULL,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
