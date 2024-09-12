CREATE TABLE
    base_happiness AS
WITH
    tmp AS (
        SELECT DISTINCT
            base_happiness AS `value`
        FROM
            opendata
        ORDER BY
            `value`
    )
SELECT
    UUID () AS `base_happiness_id`,
    `value`
FROM
    tmp;

ALTER TABLE base_happiness MODIFY COLUMN `base_happiness_id` CHAR(36) PRIMARY KEY,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
