CREATE TABLE
    classfication AS
WITH
    tmp AS (
        SELECT DISTINCT
            classfication AS `name`
        FROM
            opendata
        ORDER BY
            `name`
    )
SELECT
    UUID () AS `classfication_id`,
    `name`
FROM
    tmp;

ALTER TABLE classfication MODIFY COLUMN `classfication_id` CHAR(36) PRIMARY KEY,
MODIFY COLUMN `name` VARCHAR(100) NOT NULL,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
