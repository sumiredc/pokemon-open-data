CREATE TABLE
    percentage_male AS
WITH
    tmp AS (
        SELECT DISTINCT
            percentage_male AS `value`
        FROM
            opendata
        ORDER BY
            `value`
    )
SELECT
    UUID () AS `percentage_male_id`,
    `value`
FROM
    tmp;

ALTER TABLE percentage_male MODIFY COLUMN `percentage_male_id` CHAR(36) PRIMARY KEY,
MODIFY COLUMN `value` DECIMAL(4, 1) NOT NULL,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
