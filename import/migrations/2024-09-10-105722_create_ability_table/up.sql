CREATE TABLE
    ability AS
WITH
    a1 AS (
        SELECT
            REPLACE (
                REPLACE (REPLACE (abilities, "[", ""), "]", ""),
                "'",
                ""
            ) AS names
        FROM
            opendata
    ),
    a2 AS (
        SELECT DISTINCT
            TRIM(
                CASE n
                    WHEN 1 THEN SUBSTRING_INDEX (names, ',', 1)
                    WHEN 2 THEN SUBSTRING_INDEX (SUBSTRING_INDEX (names, ',', 2), ',', -1)
                    WHEN 3 THEN SUBSTRING_INDEX (SUBSTRING_INDEX (names, ',', 3), ',', -1)
                    WHEN 4 THEN SUBSTRING_INDEX (SUBSTRING_INDEX (names, ',', 4), ',', -1)
                    WHEN 5 THEN SUBSTRING_INDEX (SUBSTRING_INDEX (names, ',', 5), ',', -1)
                    WHEN 6 THEN SUBSTRING_INDEX (SUBSTRING_INDEX (names, ',', 6), ',', -1)
                END
            ) AS name
        FROM
            a1
            CROSS JOIN (
                SELECT
                    1 AS n
                UNION
                SELECT
                    2
                UNION
                SELECT
                    3
                UNION
                SELECT
                    4
                UNION
                SELECT
                    5
                UNION
                SELECT
                    6
            ) AS nums
    )
SELECT
    UUID () AS `ability_id`,
    name
FROM
    a2
ORDER BY
    name;

ALTER TABLE ability MODIFY COLUMN `ability_id` CHAR(36) PRIMARY KEY,
MODIFY COLUMN `name` VARCHAR(20) NOT NULL,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
