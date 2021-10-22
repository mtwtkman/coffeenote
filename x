#!/bin/sh

d="docker-compose"

_do () {
    echo "$1"
    eval "$1"
}

_run () {
    cmd="$d run --rm $@"
    _do "$cmd"
}

_exec () {
    cmd="$d exec $@"
    _do "$cmd"
}

cmd=$1
shift
case $cmd in
    build) _do "$d build --force-rm";;
    up)
        cmd="$d up -d"
        _do "$cmd"
        ;;
    down) _do "$d down";;
    log)
        cmd="$d logs -f $@"
        _do "$cmd"
        ;;
    sqlx) _run "api sqlx $@";;
    migrate) ./x sqlx "migrate --source coffeenote-api/migrations $@";;
    exec) _exec "$@";;
    psql) _exec "db psql -d coffeenote -w -U user";;
    *) $cmd $@;;
esac