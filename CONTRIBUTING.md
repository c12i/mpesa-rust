# How to Contribute

-   Create an issue or pick an already existing issue. Please avoid creating duplicate issues
-   Fork the repo, add your changes and make a pull request
-   Request for review from either me or other contributors

# How should I write my commits?

This repository follows conventional commits to enable automated releases and changelog generation.

The most important prefixes you should have in mind are:

`fix:`: represents bug fixes, and results in a SemVer patch bump.
`feat:`: represents a new feature, and results in a SemVer minor bump.
`<prefix>!`: (e.g. `feat!`:): represents a breaking change (indicated by the !) and results in a SemVer major bump.

Commits that don't follow the Conventional Commit format result in a SemVer patch bump
