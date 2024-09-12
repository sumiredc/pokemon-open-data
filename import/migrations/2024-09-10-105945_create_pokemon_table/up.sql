CREATE TABLE
    pokemon AS
SELECT
    UUID () AS `pokemon_id`,
    cl.classfication_id,
    t1.type_id AS `type1_id`,
    t2.type_id AS `type2_id`,
    g.generation_id,
    be.base_egg_steps_id,
    bh.base_happiness_id,
    e.experience_growth_id,
    cap.capture_rate_id,
    p.percentage_male_id,
    o.pokedex_number AS `number`,
    REGEXP_SUBSTR (`japanese_name`, "[ァ-ヴー♂♀]+$") AS `name`,
    REGEXP_SUBSTR (`japanese_name`, "^[A-Za-z]+") AS `kana`,
    o.name AS `english_name`,
    o.hp,
    o.attack,
    o.defense,
    o.sp_attack,
    o.sp_defense,
    o.speed,
    o.height_m,
    o.weight_kg,
    o.is_legendary
FROM
    opendata AS o
    LEFT JOIN `classfication` AS `cl` ON cl.name = o.classfication
    LEFT JOIN `type` AS `t1` ON t1.name = o.type1
    LEFT JOIN `type` AS `t2` ON t2.name = o.type2
    LEFT JOIN `generation` AS `g` ON g.value = o.generation
    LEFT JOIN `base_egg_steps` AS `be` ON be.value = o.base_egg_steps
    LEFT JOIN `base_happiness` AS `bh` ON bh.value = o.base_happiness
    LEFT JOIN `experience_growth` AS `e` ON e.value = o.experience_growth
    LEFT JOIN `capture_rate` AS `cap` ON cap.value = o.capture_rate
    LEFT JOIN `percentage_male` AS `p` ON p.value = o.percentage_male;

ALTER TABLE pokemon MODIFY COLUMN pokemon_id CHAR(36) PRIMARY KEY,
ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
ADD COLUMN `deleted_at` TIMESTAMP,
ADD CONSTRAINT fk_classfication_pokemon FOREIGN KEY (classfication_id) REFERENCES `classfication` (classfication_id),
ADD CONSTRAINT fk_type1_pokemon FOREIGN KEY (type1_id) REFERENCES `type` (type_id),
ADD CONSTRAINT fk_type2_pokemon FOREIGN KEY (type2_id) REFERENCES `type` (type_id),
ADD CONSTRAINT fk_generation_pokemon FOREIGN KEY (generation_id) REFERENCES `generation` (generation_id),
ADD CONSTRAINT fk_base_egg_steps_pokemon FOREIGN KEY (base_egg_steps_id) REFERENCES `base_egg_steps` (base_egg_steps_id),
ADD CONSTRAINT fk_base_happiness_pokemon FOREIGN KEY (base_happiness_id) REFERENCES `base_happiness` (base_happiness_id),
ADD CONSTRAINT fk_experience_growth_pokemon FOREIGN KEY (experience_growth_id) REFERENCES `experience_growth` (experience_growth_id),
ADD CONSTRAINT fk_capture_rate_pokemon FOREIGN KEY (capture_rate_id) REFERENCES `capture_rate` (capture_rate_id),
ADD CONSTRAINT fk_percentage_male_pokemon FOREIGN KEY (percentage_male_id) REFERENCES `percentage_male` (percentage_male_id);
