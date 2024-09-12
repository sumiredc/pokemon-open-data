// @generated automatically by Diesel CLI.

diesel::table! {
    ability (ability_id) {
        #[max_length = 36]
        ability_id -> Char,
        #[max_length = 20]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    base_egg_steps (base_egg_steps_id) {
        #[max_length = 36]
        base_egg_steps_id -> Char,
        value -> Unsigned<Smallint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    base_happiness (base_happiness_id) {
        #[max_length = 36]
        base_happiness_id -> Char,
        value -> Unsigned<Tinyint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    capture_rate (capture_rate_id) {
        #[max_length = 36]
        capture_rate_id -> Char,
        value -> Unsigned<Tinyint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    classfication (classfication_id) {
        #[max_length = 36]
        classfication_id -> Char,
        #[max_length = 100]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    experience_growth (experience_growth_id) {
        #[max_length = 36]
        experience_growth_id -> Char,
        value -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    generation (generation_id) {
        #[max_length = 36]
        generation_id -> Char,
        value -> Unsigned<Tinyint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    opendata (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        japanese_name -> Varchar,
        pokedex_number -> Unsigned<Smallint>,
        generation -> Unsigned<Tinyint>,
        #[max_length = 10]
        type1 -> Varchar,
        #[max_length = 10]
        type2 -> Nullable<Varchar>,
        base_total -> Unsigned<Smallint>,
        hp -> Unsigned<Tinyint>,
        attack -> Unsigned<Tinyint>,
        defense -> Unsigned<Tinyint>,
        sp_attack -> Unsigned<Tinyint>,
        sp_defense -> Unsigned<Tinyint>,
        speed -> Unsigned<Tinyint>,
        abilities -> Text,
        base_egg_steps -> Unsigned<Smallint>,
        base_happiness -> Unsigned<Tinyint>,
        capture_rate -> Unsigned<Tinyint>,
        #[max_length = 100]
        classfication -> Varchar,
        experience_growth -> Nullable<Unsigned<Integer>>,
        percentage_male -> Nullable<Decimal>,
        height_m -> Nullable<Unsigned<Decimal>>,
        weight_kg -> Nullable<Unsigned<Decimal>>,
        is_legendary -> Bool,
        against_bug -> Unsigned<Decimal>,
        against_dark -> Unsigned<Decimal>,
        against_dragon -> Unsigned<Decimal>,
        against_electric -> Unsigned<Decimal>,
        against_fairy -> Unsigned<Decimal>,
        against_fight -> Unsigned<Decimal>,
        against_fire -> Unsigned<Decimal>,
        against_flying -> Unsigned<Decimal>,
        against_ghost -> Unsigned<Decimal>,
        against_grass -> Unsigned<Decimal>,
        against_ground -> Unsigned<Decimal>,
        against_ice -> Unsigned<Decimal>,
        against_normal -> Unsigned<Decimal>,
        against_poison -> Unsigned<Decimal>,
        against_psychic -> Unsigned<Decimal>,
        against_rock -> Unsigned<Decimal>,
        against_steel -> Unsigned<Decimal>,
        against_water -> Unsigned<Decimal>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    percentage_male (percentage_male_id) {
        #[max_length = 36]
        percentage_male_id -> Char,
        value -> Decimal,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pokemon (pokemon_id) {
        #[max_length = 36]
        pokemon_id -> Char,
        #[max_length = 36]
        classfication_id -> Nullable<Char>,
        #[max_length = 36]
        type1_id -> Nullable<Char>,
        #[max_length = 36]
        type2_id -> Nullable<Char>,
        #[max_length = 36]
        generation_id -> Nullable<Char>,
        #[max_length = 36]
        base_egg_steps_id -> Nullable<Char>,
        #[max_length = 36]
        base_happiness_id -> Nullable<Char>,
        #[max_length = 36]
        experience_growth_id -> Nullable<Char>,
        #[max_length = 36]
        capture_rate_id -> Nullable<Char>,
        #[max_length = 36]
        percentage_male_id -> Nullable<Char>,
        number -> Unsigned<Smallint>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        kana -> Nullable<Varchar>,
        #[max_length = 100]
        english_name -> Varchar,
        hp -> Unsigned<Tinyint>,
        attack -> Unsigned<Tinyint>,
        defense -> Unsigned<Tinyint>,
        sp_attack -> Unsigned<Tinyint>,
        sp_defense -> Unsigned<Tinyint>,
        speed -> Unsigned<Tinyint>,
        height_m -> Nullable<Unsigned<Decimal>>,
        weight_kg -> Nullable<Unsigned<Decimal>>,
        is_legendary -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    #[sql_name = "type"]
    type_ (type_id) {
        #[max_length = 36]
        type_id -> Char,
        #[max_length = 10]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(pokemon -> base_egg_steps (base_egg_steps_id));
diesel::joinable!(pokemon -> base_happiness (base_happiness_id));
diesel::joinable!(pokemon -> capture_rate (capture_rate_id));
diesel::joinable!(pokemon -> classfication (classfication_id));
diesel::joinable!(pokemon -> experience_growth (experience_growth_id));
diesel::joinable!(pokemon -> generation (generation_id));
diesel::joinable!(pokemon -> percentage_male (percentage_male_id));

diesel::allow_tables_to_appear_in_same_query!(
    ability,
    base_egg_steps,
    base_happiness,
    capture_rate,
    classfication,
    experience_growth,
    generation,
    opendata,
    percentage_male,
    pokemon,
    type_,
);
