# Jira Branches

Second iteration of Jira Branches implemented with Rust for improved performance.

Jira branches is a simple CLI tool for using Jira to define git branches. Git branch name patterns can be defined with
tokens representing attributes of Jira tickets.

For example: `feature/PROJECT-1078-some-jira-ticket-title`

---

# Installation

## Mac

1. Download the latest release

   #### Apple Silicon
   ```bash
   curl -OL https://github.com/jtstott/jira-branches/releases/latest/download/jb-aarch64-apple-darwin.tar.gz && \
   tar -xzvf jb-aarch64-apple-darwin.tar.gz && \
   rm jb-aarch64-apple-darwin.tar.gz
   ```

   #### Intel
   ```bash
   curl -OL https://github.com/jtstott/jira-branches/releases/latest/download/jb-x86_64-apple-darwin.tar.gz && \
   tar -xzvf jb-x86_64-apple-darwin.tar.gz && \
   rm jb-x86_64-apple-darwin.tar.gz
   ```

2. The `jb` binary will be available in your current directory, move it to a directory in your path, for
   example `/usr/local/bin/` (requires sudo):
   ```bash
   sudo mv jb /usr/local/bin/jb
   ```

## Linux

1. Download the latest release

   #### Intel
   ```bash
   curl -OL https://github.com/jtstott/jira-branches/releases/latest/download/jb-x86_64-unknown-linux-gnu.tar.gz && \
   tar -xzvf jb-x86_64-unknown-linux-gnu.tar.gz && \
   rm jb-x86_64-unknown-linux-gnu.tar.gz
   ```

2. The `jb` binary will be available in your current directory, move it to a directory in your path, for
   example `/usr/local/bin/` (requires sudo):
   ```bash
   sudo mv jb /usr/local/bin/jb
   ```

---

# Setup

There are two ways to configure jira branches, either [manually](#manual) by creating the config files, or by using the
[config wizard](#config-wizard).

## Config wizard

Jira branches can be configured through the CLI with the `configure` command:

```shell
jb configure
```

Enter the values when prompted, refer to the configuration [section](#configuration) for further details on each of the
config values.

## Manual

1. Authenticate with Jira by creating the file `~/.config/jira-branches/auth.json` and add your Jira username (email)
   and
   password (this can be a Jira API token):
   ```json lines
   {
     // Jira username
     "user": "",
     // Jira password
     "password": ""
   }
   ```

2. Define your configuration in the file: `~/.config/jira-branches/config.json`. The configuration should be in the
   following format:
   ```json lines
   {
     // Jira instance base URL
     "base_url": "",
     // Branch name template with jira variable tokens
     "branch_template": "",
     "options": {
       "id_prefix": "",
       "map_types": {},
       "case": {}
     }
   }
   ```

The valid configuration options are described in the configuration [section](#configuration). You can use
the [full example configuration file](#full-example-configuration-file) as a starting point.

---

# Usage

Assuming the binary is in your local PATH, then the tool can be run from your terminal with the `jb` command.
Check that this works by running

```shell
jb --help
```

## Checkout branch

The `checkout` command takes an `ISSUE ID` argument, and will checkout a git branch for the current git repository
formatted to the configured template. If the branch doesn't already exist it will be created.

To checkout to a git branch for a given Jira ticket ID (`ID`), or Jira ticket URL, run the `checkout` command, supplying
the ticket ID or ticket URL:

```shell
jb checkout <ISSUE>
```

### Alias

To create a shorthand for this command, create an alias in your `.bash_profile` (or whichever shell profile you use) to
the checkout
command to make it easier to use:

```bash
alias jbc="jb checkout"
```

The checkout command can easily be run then using the following shorthand:

```bash
jbc ID
```

## List branches

To list the available branches for a given Jira issue ID, or issue URL, you can use the list command:

```shell
jb ls <ISSUE>
```

This will return a list of any branches that include the given issue ID.

*_This command is only able to search through branches that are available on your local machine. To ensure the branches
are up-to-date with the remote, update your local before running this command, for example by running `git pull`._

---

# Configuration

## Required

| Key               | Description                                                                                                                                                                                                             |
|-------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `base_url`        | Jira base URL. This will be the base URL of your Jira instance.<br/>For example, https://my-org.atlassian.net                                                                                                           |
| `branch_template` | The `branch_template` must be configured to the format the branch name will be created to. Any string is valid (excluding forbidden branch name characters), and tokens can be used to represent Jira ticket variables. |

## Options

| Option      | Description                                                                                                                                                                                                                                                                                                                                                                          |
|-------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `id_prefix` | The `id_prefix` option can be used if all Jira ticket IDs start with the same prefix, meaning the prefix can be omitted when the tool is executed.<br/><br/>For example if a all ticket IDs start with the prefix `ATEAM-`, and the `id_prefix` is configured to this, then branches can be created for a ticket with ID  `ATEAM-1078` by just passing `1078` as the issue argument. |
| `map_types` | The `map_types` option allows jira ticket types to be mapped to other values for branch name generation. For example, a ticket type of `Story` may want to be mapped in the branch name to `feature`.                                                                                                                                                                                |
| `case`      | The `case` option can be used to transform the raw Jira values to either upper or lower case. Valid values are either `"upper"` or `"lower"`.                                                                                                                                                                                                                                        |

## Branch template

The `branch_template` must be configured to the format the branch name will be created to. Any string is valid (
excluding
forbidden branch name characters), and tokens can be used to represent Jira ticket variables.

Jira ticket variables can be used in the name template using the following syntax: `[var]`

For example, given a ticket has an ID '_ID-123_' and a summary '_Great feature_', the following branch
template: `"feat/[id]-[summary]"` would generate the branch name `feat/ID-123-great-feature`.

It is important to note that spaces will always be replaced by dashes (`-`).

### Valid jira ticket variables

The branch name tokens currently supported are:

- `id`: Ticket ID
- `summary`: Ticket summary/title
- `type`: Ticket type (e.g. story, task, bug etc...)

## Map types

The `mapTypes` config option allows jira ticket types to be mapped to other values for branch name generation. For
example a ticket type of `Story` may want to be mapped in the branch name to `feature`.

### Example

```json
{
  "options": {
    "map_types": {
      "Story": "feature",
      "*": "task"
    }
  }
}
```

This mapping will be invoked anytime the ticket type variable is used in the branch template.

The wildcard `*` can be used as a default case. If a key of `*` is set for `map_types`, the value set for this default
case will be applied to the type variable when no other type mappings are matched. Without a default case, if
no `map_type` mappings are found then the original value for the ticket type is used.

## Case

The case option can be used to transform the real Jira values to either upper or lower case. Valid values are
either `"upper"` or `"lower"`.

### Example

```json
{
  "options": {
    "case": {
      "type": "lower",
      "summary": "lower"
    }
  }
}
```

## Full example configuration file

This is an example of a complete `~/.config/jira-branches/config.json` with all options applied.

```json
{
  "base_url": "https://my-org.atlassian.net",
  "branch_template": "feat/[id]-[summary]",
  "options": {
    "id_prefix": "ATEAM-",
    "map_types": {
      "Story": "feature",
      "Bug": "fix",
      "*": "task"
    },
    "case": {
      "type": "upper",
      "summary": "lower"
    }
  }
}
```
