create table shops (
    id uuid primary key default gen_random_uuid()
    ,name text not null
    ,url text not null default ''
    ,created_at timestamp not null default now()
    ,updated_at timestamp not null default now()
)
;

create table regions (
    id uuid primary key default gen_random_uuid()
    ,name text not null
    ,created_at timestamp not null default now()
    ,updated_at timestamp not null default now()
)
;

create table production_areas (
    id uuid primary key default gen_random_uuid()
    ,name text not null
    ,region_id uuid references regions (id) not null
    ,created_at timestamp not null default now()
    ,updated_at timestamp not null default now()
)
;

create table beans (
    id uuid primary key default gen_random_uuid()
    ,name text not null
    ,shop_id uuid references shops (id)
    ,production_area_id uuid references production_areas (id) not null
    ,purchased_at timestamp not null
    ,created_at timestamp not null default now()
    ,updated_at timestamp not null default now()
    ,unique (name, shop_id)
)
;

create type roast_degree as enum (
    'light_city'
    ,'half_city'
    ,'cinnamon'
    ,'new_england'
    ,'regular'
    ,'american'
    ,'city'
    ,'breakfast'
    ,'high_city'
    ,'full_city'
    ,'high_full_city'
    ,'after_dinner'
    ,'vienna'
    ,'french'
    ,'italian'
    ,'espresso'
    ,'continental'
    ,'new_orleans'
    ,'spanish'
)
;

create table roasts (
    id uuid primary key default gen_random_uuid()
    ,bean_id uuid references beans (id) not null
    ,first_cracked_seconds integer not null
    ,second_cracked_seconds integer not null
    ,degree roast_degree not null
    ,note text not null
    ,roasted_at timestamp not null default now()
    ,updated_at timestamp not null default now()
)
;