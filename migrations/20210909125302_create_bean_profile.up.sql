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
    ,region_id uuid references regions (id)
    ,created_at timestamp not null default now()
    ,updated_at timestamp not null default now()
)
;

create table beans (
    id uuid primary key default gen_random_uuid()
    ,name text not null
    ,shop_id uuid references shops (id)
    ,production_area_id uuid references production_areas (id)
    ,purchased_at timestamp not null
    ,created_at timestamp not null default now()
    ,updated_at timestamp not null default now()
    ,unique (name, shop_id)
)
;