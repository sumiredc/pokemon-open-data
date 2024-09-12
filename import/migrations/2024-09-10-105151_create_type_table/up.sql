CREATE TABLE
    `type` AS
WITH
    tmp AS (
        SELECT
            type1 AS `name`
        FROM
            opendata
        UNION
        SELECT
            type2 AS `name`
        FROM
            opendata
        WHERE
            type2 <> ""
    )
SELECT
    UUID () AS `type_id`,
    `name`
FROM
    tmp;

ALTER TABLE `type` MODIFY COLUMN `type_id` CHAR(36) PRIMARY KEY,
MODIFY COLUMN `name` VARCHAR(10) NOT NULL,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
