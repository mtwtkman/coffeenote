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
    build) _do "COMPOSE_DOCKER_CLI_BUILD=1 $d build --force-rm";;
    up)
        cmd="$d up -d"
        _do "$cmd"
        ;;
    down) _do "$d down";;
    restart) _do "$d restart";;
    log)
        cmd="$d logs -f $@"
        _do "$cmd"
        ;;
    grpc)
        schema=$1
        data=$2
        symbol=$3
        GODEBUG=x509ignoreCN=0 grpcurl \
            -insecure \
            -cert ./coffeenote-api/tls/cert.pem \
            -key ./coffeenote-api/tls/key.pem \
            -import-path ./coffeenote-api/proto \
            -proto $schema.proto \
            -d '$data' \
            localhost:55301 $symbol
        ;;
    sqlx) _run "api sqlx $@";;
    migrate) ./x sqlx "migrate --source coffeenote-api/migrations $@";;
    exec) _exec "$@";;
    run) _run "$@";;
    psql) _exec "db psql -d coffeenote -w -U user";;
    test|t) _exec "api cargo test $@";;
    *) $cmd $@;;
esac