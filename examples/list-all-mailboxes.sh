#! /usr/bin/env bash
set euo pipefail

printf "List all mailboxes\n"

cargo run -- \
        --user "your.username@email.com" \
        --pass "your-api-token" \
        --domain "your-domain.xyz" \
        mailboxes index