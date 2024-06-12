export PYTHONPATH=${PROJDIR}/dj


DATABASE_NAME=stream_question
ROLE_NAME=stream_question_user
PASSWORD=temp_pass

export DATABASE_URL=postgres://${ROLE_NAME}:${PASSWORD}@127.0.0.1:5432/${DATABASE_NAME}

function pushd2() {
  PUSHED="$(pwd)"
  cd "$PROJDIR""$1" || return 0
}

function popd2() {
  cd "${PUSHED:-$PROJDIR}" || return 0
  unset PUSHED
}

function manage() {
  pushd2 /dj
  python3 manage.py $*
  r=$?
  popd2
  return ${r}
}

function recreatedb() {
  WHO=`whoami`
  psql -h 127.0.0.1 -U "${WHO}" -d postgres -c "DROP ROLE IF EXISTS ${ROLE_NAME};"
  psql -h 127.0.0.1 -U "${WHO}" -d postgres -c "CREATE ROLE ${ROLE_NAME} WITH SUPERUSER LOGIN PASSWORD '${PASSWORD}';"
  psql -h 127.0.0.1 -U "${WHO}" -d postgres -c "DROP DATABASE IF EXISTS ${DATABASE_NAME};"
  psql -h 127.0.0.1 -U "${WHO}" -d postgres -c "CREATE DATABASE ${DATABASE_NAME};"
  migrate $*
}

function pyfmt() {
  pushd2 /dj
  black .
  popd2
}
