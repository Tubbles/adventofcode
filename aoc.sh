#!/bin/bash

my_dir="$(dirname "$(realpath "$0")")"
cookie_file="${my_dir}/cookie"
cookie="$(cat "${cookie_file}")"

usage="Advent of Code helper
$(basename "$0") <year> <day>
$(basename "$0") <year> <day> input
$(basename "$0") <year> <day> answer <part> <answer>"

year="$1"
day="$2"
action="$3"
part="$4"
answer="$5"

endpoint="https://adventofcode.com/${year}/day/${day}"
flags="--cookie session=${cookie} --silent --show-error"

if [[ "$1" == "help" || "$1" == "--help" || "$1" == "-h" || "$1" == "/?" || "$1" == "?" ]]; then
    echo "${usage}"
elif [[ -z "${action}" && -n ${year} && -n ${day} ]]; then
    # Word splitting intended
    # shellcheck disable=SC2086
    curl ${flags} "${endpoint}" | grep -A9999 "<article" | grep -B9999 "</article>" | html2text
elif [[ "${action}" == "input" ]]; then
    # Word splitting intended
    # shellcheck disable=SC2086
    curl ${flags} "${endpoint}/input"
elif [[ "${action}" == "answer" ]]; then
    # Word splitting intended
    # shellcheck disable=SC2086
    curl ${flags} "${endpoint}/answer" --data-raw "level=${part}&answer=${answer}" | grep -A9999 "<article" | grep -B9999 "</article>" | html2text
elif [[ "${action}" == "input" ]]; then
    # Word splitting intended
    # shellcheck disable=SC2086
    curl ${flags} "${endpoint}/input"
else
    echo "${usage}"
    exit 1
fi
