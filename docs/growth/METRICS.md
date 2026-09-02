# Growth metrics

Growth is a system metric, not a vanity-number contest.

## Funnel

```text
search impression
  → site/repository visit
  → install or local setup
  → successful connection
  → star/fork
  → issue/PR
  → repeat contribution
```

## Track

- repository stars and forks
- unique release downloads
- documentation page traffic
- guide-to-repository click-through
- issue creation rate
- PR creation and merge rate
- time from issue to reproducible test
- regression rate after releases
- number of independent deployment reports

## Quality guardrails

Do not optimize for clicks at the expense of technical trust. A guide that generates traffic but causes unsafe configuration is a failed asset.

## Weekly feedback loop

1. Identify the most common real user problem.
2. Fix or document it.
3. Add a regression test.
4. Publish the evidence.
5. Update the relevant guide.
6. Link the guide from the README.
7. Review whether support burden decreased.

This makes growth and engineering reinforce each other instead of competing.
