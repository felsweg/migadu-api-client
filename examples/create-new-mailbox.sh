#! /usr/bin/env bash
set euo pipefail

printf "Creates a new mailbox\n"

cargo run -- \
        --user "your.username@email.com" \
        --pass "your-api-token" \
        --domain "your-domain.xyz" \
        mailboxes create \
        --name "Your fancy mailbox name" \
        --local_part "fancy-mail-name" \
        --password_method "invitation"
        --password_recovery_email "recovery@email.com"
