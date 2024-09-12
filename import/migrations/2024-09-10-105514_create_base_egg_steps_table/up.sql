CREATE TABLE
    base_egg_steps AS
WITH
    tmp AS (
        SELECT DISTINCT
            base_egg_steps AS `value`
        FROM
            opendata
        ORDER BY
            `value`
    )
SELECT
    UUID () AS `base_egg_steps_id`,
    `value`
FROM
    tmp;

ALTER TABLE base_egg_steps MODIFY COLUMN `base_egg_steps_id` CHAR(36) PRIMARY KEY,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
