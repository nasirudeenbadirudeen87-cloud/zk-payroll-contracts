# Privacy-safe payroll event schema

The payroll contracts publish a versioned indexer stream for lifecycle events.
Consumers that do not need the legacy compatibility stream should filter events
whose topics have the following shape:

```text
(payroll, event_name, 2, entity_hash)
```

The non-indexed payload is always:

```text
(count, reason_code)
```

Schema version `2` deliberately does not contain salary amounts, employee
addresses, employee metadata, or payroll rows. Address and numeric identifiers
are represented by one-way SHA-256 hashes where they are used as an entity key.

| Event name | Entity hash | Count | Reason code |
| --- | --- | --- | --- |
| `commitment` | commitment hash | `1` | `updated` |
| `lock` | employee-address hash | `1` | `locked` or `unlocked` |
| `execution` | payroll-run-id hash | employee count | `executed` |
| `settlement` | payroll-run-id hash | `1` | `reconciled`, `unreconciled`, or `failed` |
| `cancellation` | payroll-run-id hash | `1` | caller-supplied cancellation reason |
| `audit_grant` | auditor-address hash | expiration ledger | `granted` |
| `treasury_readiness` | asset-address hash | `1` or `0` | `ready` or `not_ready` |

The existing legacy events remain available for backward compatibility with
older SDKs. They are not the privacy-safe contract: indexers should migrate to
the versioned stream above and must not persist sensitive fields from legacy
events as part of the normalized payroll schema.

Lifecycle emissions are ordered by the contract execution that produces them.
For a completed run, consumers can therefore observe the execution event before
the settlement event; cancellation emits its cancellation event after the run
is marked cancelled.
