# Design

This document describes the design of MonoRelease.

It is built for monorepos.

Inspired by changesets, it also uses change files describing the changes that have been made.

It uses a global configuration file `monorelease.yaml` at the root of the repository where it can be configured.
It uses a `releases/` folder (the name of which can be configured in `monorelease.yaml`) for state and data.
This contains a `waves/` folder which essentially is the version of the monorepo as a whole.
This version is simply an incrementing integer, for example, `42`.
The `releases/` folder also contains a `next/` folder, which contains all the change files that are not a part of a wave yet.

Whenever any project is released, a new wave is created that contains all the change files not yet released.
For example, if the next wave is 63, then a folder named `63/` is created in the `waves/` folder, and the changes from `next/` are moved to this new wave folder.
The waves are simply used to track the version of the monorepo.

The metadata and version of the projects are instead tracked in the `releases/projects/` folder.
This contains a file for each project, for example, `open-game-rules.yaml`.
The ID of the project is the file name (excluding the file extension).
This file also contains the display name of the project, and a history of the versions.
This history is a simple map from a version of the project to a wave.
Thus, it might map that version `1.2.3` is wave 8.
This helps monorelease ensure that despite later alterations to old change files, it will provide a constantly increasing version number,
while still being able to generate a change log with references to the correct version numbers for the right changes (despite the descriptions or magnitude of the changes having been altered in the change files).

The metadata file for the project also contains a list of the projects that this project is dependent on.
This dependency mapping is used to bump the version of the project if a dependency project gets a new version.
The dependencies are a map from an project ID to an object.
Currently, this object can only hold one key, which is `magnitude`, which can either be "patch" (default), "minor", "major", or "match".
The magnitude key sets how a change in the dependency should affect the current project.
For example, when set to "patch", any version bump of any magnitude will result in a patch bump on the current project.
When using the "match" value, the magnitude of the dependency's version bump is directly reflected in the version bump of the current project.

Monorelease has two separate processes: one for creating a new wave, and another for creating a release of an project to a specific wave.
By separating these two processes, it is much easier to separate the cadence of releases for various projects.
This is useful for example, when implementing a PR-based release process, because you can have the PR be that it creates a new wave.
Then, on every push to the main branch, you have monorelease check for pending updates (excluding the `next/` folder) for an project (and have monorelease create the release) if it needs to be released whenever the PR is merged.
Or, you can have monorelease check for these updates (and create the release) every two weeks.
Thus, you may have one project release whenever possible, and another can release every two weeks, or on a manual trigger.

Monorelease can also be used to generate changelogs.
Multiple options exist, such as generating a complete changelog of all files, or simply generating a changelog of only the most recent version, or a specified version.

To find pending changes, MonoRelease looks at the highest wave number in the project's history map (e.g., Wave 8).
It then scans all waves from Wave 9 up to the current highest wave to calculate the next version bump.

Releasing an project simply means MonoRelease calculates the new version and appends a new key-value pair to the history map in `releases/projects/<project-id>.yaml` (e.g., adding `"1.3.0": 42`)

## Change Files

These are markdown files with YAML frontmatter.

Example:

```markdown
---
"my-first-app": minor
"my-third-app": patch
---
Added the new scoring mechanic for overtime.
```

## Possible Future Additions

When creating a new wave, it is possible to choose one or more projects whose release changes should be added to the wave.
This would make it such that creating a wave doesn't necessarily mark all projects for release.
However, how it should be handled if a single change file affects multiple projects and one or more shouldn't be included in the wave is unclear.
Perhaps the solution is simply to return an error, and have the user manually split the change file or include the project manually.
One solution might also be to simply include all that project's change files as well then (if the user approves), but I think this is a bad approach compared to simply throwing an error because this approach might lead to accidental releases.
