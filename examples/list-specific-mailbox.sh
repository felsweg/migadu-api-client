#! /usr/bin/env bash
set euo pipefail

printf "List a specific mailbox\n"

cargo run -- \
        --user "your.username@email.com" \
        --pass "your-api-token" \
        --domain "your-domain.xyz" \
        --extra "mailbox-name" \
        mailboxes show