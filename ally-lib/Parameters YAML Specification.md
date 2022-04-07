---
tags:
- rust
- ally
- specification
---

# Parameters YAML Specification

## Notes

### Boolean Values
- Whenever a boolean value is mentioned below, possible values for **true** and **false** respectively are:
	- True/False
	- On/Off
	- Yes/No

## Commands

Commands represent the primary command to be run. They are specified with a mapping of setting, like this:

```yaml
command: {
	name: <val>,
	args: [
		arg: {
			# ...
		},
		# ...
	],
	subcommands: [
	    command: {
			# ...
		},
		# ...
	],
}
```

The valid set of settings are as follows:

- `name` (string): A unique name for the command, usually the name of the program.
- `args` ([[#Arguments]] array): The args list for this command.
- `subcommands` ([[#Commands]] array): The list of commands that operate as subcommands of this command.
- `about` (string): Sets the program's description for the short help (`-h`).
- `after_help` (string): Free-form help text for after auto-generated short help (`-h`). This is often used to describe how to use the arguments, caveats to be noted, or license and contact information.
- `after_long_help` (string): Free-form help text for after auto-generated long help (`--help`). This is often used to describe how to use the arguments, caveats to be noted, or license and contact information.
- `alias` (string): Set a hidden alias for this subcommand.
- `allow_external_subcommands` ([[#Boolean Values|boolean]]): Assume unexpected positional args are a subcommand.
- `allow_hyphen_values` ([[#Boolean Values|boolean]]): Specifies that leading hyphens are allowed in all arg *values* (e.g. `-10`).
- `allow_invalid_utf8_for_external_subcommands` ([[#Boolean Values|boolean]]): Specifies that external subcommands that are invalid UTF-8 should *not* be treated as an error.
- `allow_missing_positional` ([[#Boolean Values|boolean]]): Allows one to implement two styles of CLIs where positionals can be used out of order.

	The first example is a CLI where the second to last positional argument is optional, but the final positional argument is required. Such as `$ prog [optional] <required>` where one of the two following usages is allowed:

    > -   `$ prog [optional] <required>`
    > -   `$ prog <required>`

	This would otherwise not be allowed. This is useful when `[optional]` has a default value.

	**Note:** when using this style of “missing positionals” the final positional *must* be `required` if `--` will not be used to skip to the final positional argument.

	**Note:** This style also only allows a single positional argument to be “skipped” without the use of `--`.

	For example, imagine a CLI which has three positional arguments `[foo] [bar] [baz]...` where `baz` accepts multiple values (similar to man `ARGS...` style training arguments).

	With this setting the following invocations are posisble:

    > -   `$ prog foo bar baz1 baz2 baz3`
    > -   `$ prog foo -- baz1 baz2 baz3`
    > -   `$ prog -- baz1 baz2 baz3`
- `allow_negative_numbers` ([[#Boolean Values|boolean]]): Allow negative numbers to pass as values.
- `args` ([[#Arguments]]): A sequence of args for the command.
- `arg_required_else_help` ([[#Boolean Values|boolean]]): Exit gracefully if no arguments are present.
- `args_conflicts_with_subcommands` ([[#Boolean Values|boolean]]): Specifies that use of an argument prevents the use of [[#Subcommands]].

	By default, arguments between subcommands such as `<cmd> [cmd_args] <subcmd> [subcmd_args] <subsubcmd> [subsubcmd_args]` are allowed.

	This setting disables that functionality and says that arguments can only follow the *final* subcommand. For instance using this setting makes only the following invocations possible:

	> -   `<cmd> <subcmd> <subsubcmd> [subsubcmd_args]`
	> -   `<cmd> <subcmd> [subcmd_args]`
	> -   `<cmd> [cmd_args]`
- `args_override_self` ([[#Boolean Values|boolean]]): Specifies that all arguments override themselves.
- `author` (string): Sets the author(s) for the help message.
- `before_help` (string): Free-form help text for before auto-generated short help (`-h`).

	This is often used for header, copyright, or license information.
- `before_long_help` (string): Free-form help text for before auto-generated long help (`--help`).

	This is often used for header, copyright, or license information.
- `bin_name` (string): Overrides the runtime-determined name of the binary for help and error messages.

	This should only be used when absolutely necessary, such as when the binary name for your application is misleading, or perhaps *not* how the user should invoke your program.
	
	**Pro-tip:** When building things such as third party `cargo` subcommands, this setting **should** be used!
	
	**NOTE:** This _does not_ change or set the name of the binary file on disk. It only changes what clap thinks the name is for the purposes of error or help messages.
- `disable_colored_help` ([[#Boolean Values|boolean]]): Disables colorized help messages.
- `disable_help_flag` ([[#Boolean Values|boolean]]): Disables the `-h` and `--help` flags.
- `disable_help_subcommand` ([[#Boolean Values|boolean]]): Disables the `help` subcommand.
- `disable_version_flag` ([[#Boolean Values|boolean]]): Disables the `-V` and `--version` flags.
- `display_order` (positive number): Set the placement of this subcommand within the help.

	Subcommands with a lower value will be displayed first in the help message. Subcommands with duplicate display orders will be displayed in alphabetical order.
	
	This is helpful when one would like to emphasize frequently used subcommands, or prioritize those towards the top of the list.
	
	**NOTE:** The default is 999 for all subcommands.
- `dont_collapse_args_in_usage` ([[#Boolean Values|boolean]]): Disables the automatic collapsing of positional args into `[ARGS]` inside the usage string.
- `dont_delimit_trailing_values` ([[#Boolean Values|boolean]]): Disables the automatic delimiting of values after `--`.
- `group` ([[##ArgGroup]]): Adds an `ArgGroup` to the application.

	`ArgGroup`s are a family of related arguments. By placing them in a logical group, you can build easier requirement and exclusion rules.
	
	Example use cases:
	
	-   Make an entire `ArgGroup` required, meaning that one (and *only* one) argument from that group must be present at runtime.
	-   Name an `ArgGroup` as a conflict to another argument. Meaning any of the arguments that belong to that group will cause a failure if present with the conflicting argument.
	-   Ensure exclusion between arguments.
	-   Extract a value from a group instead of determining exactly which argument was used.
- `help_expected` ([[#Boolean Values|boolean]]): Panic if help descriptions are omitted.
- `help_template` (string): Sets the help template to be used, overriding the default format.

	**NOTE:** The template system is by design very simple. Therefore, the tags have to be written in the lowercase and without spacing.
	
	Tags are given inside curly brackets.
	
	Valid tags are:
	
	-   `{bin}` - Binary name.
	-   `{version}` - Version number.
	-   `{author}` - Author information.
	-   `{author-with-newline}` - Author followed by `\n`.
	-   `{author-section}` - Author preceded and followed by `\n`.
	-   `{about}` - General description `about` or `long_about`.
	-   `{about-with-newline}` - About followed by `\n`.
	-   `{about-section}` - About preceded and followed by `\n`.
	-   `{usage-heading}` - Automatically generated usage heading.
	-   `{usage}` - Automatically generated or given usage string.
	-   `{all-args}` - Help for all arguments (options, flags, positional arguments, and subcommands) including titles.
	-   `{options}` - Help for options.
	-   `{positionals}` - Help for positional arguments.
	-   `{subcommands}` - Help for subcommands.
	-   `{after-help}` - Help from `after_help` or `after_long_help`.
	-   `{before-help}` - Help from `before_help` or `before_long_help`.
- `hide` ([[#Boolean Values|boolean]]): Specifies that this subcommand should be hidden from help messages.
- `hide_possible_values` ([[#Boolean Values|boolean]]): Tells command *not* to print possible values when displaying help information.
	
	This can be useful if there are many values, or they are explained elsewhere.
- `ignore_errors` ([[#Boolean Values|boolean]]): Try not to fail on parse errors, like missing option values.
- `infer_long_args` ([[#Boolean Values|boolean]]): Allow partial matches of long arguments or their aliases.
	
	For example, to match an argument named `--test`, one could use `--t`, `--te`, `--tes`, and `--test`.
	
	**NOTE:** The match *must not* be ambiguous at all in order to succeed. i.e. to match `--te` to `--test` there could not also be another argument or alias `--temp` because both start with `--te`.
- `infer_subcommands` ([[#Boolean Values|boolean]]): Allow partial matches of subcommand names and their aliases.

	For example, to match a subcommand named `test`, one could use `t`, `te`, `tes`, and `test`.
	
	**NOTE:** The match *must not* be ambiguous at all in order to succeed. i.e. to match `te` to `test` there could not also be a subcommand or alias `temp` because both start with `te`.
	
	**CAUTION:** This setting can interfere with positional/free arguments. Take care when designing CLIs which allow inferred subcommands and have potential positional/free arguments whose values could start with the same characters as subcommands. If this is the case, it’s recommended to use settings such as `args_conflicts_with_subcommands` in conjunction with this setting.
- `long_about` (string): Sets the program’s description for the long help (`--help`).
	
	If `about` is not specified, this message will be displayed for `-h`.
- `long_flag` (string): Sets the long version of the subcommand flag without the preceding `--`.

	Allows the subcommand to be used as if it were an `long`.

	**NOTE:** Any leading `-` characters will be stripped.
- `long_flag_alias` (string): Add an alias, which functions as a “hidden” long flag subcommand.

	This will automatically dispatch as if this subcommand was used. This is more efficient, and easier than creating multiple hidden subcommands as one only needs to check for the existence of this command, and not all variants.
- `long_version` (string): Sets the version for the long version (`--version`) and help messages.
- `max_term_width` (non-negative number): Sets the maximum terminal width at which to wrap help messages.

	This only applies when setting the current terminal width. See `term_width` for more details.

	Using `0` will ignore terminal widths and use source formatting.

	**NOTE:** This setting applies globally and *not* on a per-command basis.
- `next_help_heading` (string): Set the default section heading for future args.

	This will be used for any arg that hasn’t had `help_heading` called.

	This is useful if the default `OPTIONS` or `ARGS` headings are not specific enough for one’s use case.
- `next_line_help` ([[#Boolean Values|boolean]]): Places the help string for all arguments and subcommands on the line after them.
- `no_binary_name` ([[#Boolean Values|boolean]]): Specifies that the parser should not assume the first argument passed is the binary name.

	This is normally the case when using a “daemon” style mode, or an interactive CLI where one would not normally type the binary or program name for each command.
- `override_help` (string): Overrides generated help message (both `-h` and `--help`).

	This should only be used when the auto-generated message does not suffice.
- `override_usage` (string): Overrides the generated usage string for help and error messages.
- `propagate_version` ([[#Boolean Values|boolean]]): Specifies to use the version of the current command for all subcommands.

	Defaults to false; subcommands have independent version strings from their parents.

	**Note:** Make sure you apply it as `global_setting` if you want this setting to be propagated to subcommands and sub-subcommands!

	**NOTE:** This choice is propagated to all child subcommands.
- `short_flag` (single character): Sets the short version of the subcommand flag without the preceding `-`.
- `short_flag_alias` (single character): Add an alias, which functions as “hidden” short flag subcommand

	This will automatically dispatch as if this subcommand was used. This is more efficient, and easier than creating multiple hidden subcommands as one only needs to check for the existence of this command, and not all variants.
- `subcommand_help_heading` (string): Sets the help heading used for subcommands when printing usage and help.

	By default, this is “SUBCOMMANDS”.
- `subcommand_negates_reqs` ([[#Boolean Values|boolean]]): Allows subcommands to override all requirements of the parent command.

	For example, if you had a subcommand or top level application with a required argument that is only required as long as there is no subcommand present, using this setting would allow you to set those arguments to [[#Arguments]] `required: true` and yet receive no error so long as the user uses a valid subcommand instead.

	**NOTE:** This defaults to false (using subcommand does *not* negate requirements)
- `subcommand_precedence_over_arg` ([[#Boolean Values|boolean]]): Prevent subcommands from being consumed as an arg value.

	By default, if an option taking multiple values is followed by a subcommand, the subcommand will be parsed as another value.

	```
	cmd --foo val1 val2 subcommand
			  --------- ----------
				values   another value
	```

	This setting instructs the parser to stop when encountering a subcommand instead of greedily consuming arguments.

	```
	cmd --foo val1 val2 subcommand
			  --------- ----------
				values   subcommand
	```

	**Note:** Make sure you apply it as `global_setting` if you want this setting to be propagated to subcommands and sub-subcommands!
- `subcommand_required` ([[#Boolean Values|boolean]]): If no subcommand is present at runtime, error and exit gracefully.
- `subcommand_value_name` (string): Sets the value name used for subcommands when printing usage and help.

	By default, this is “SUBCOMMAND”.
- `term_width` (non-negative number): Sets the terminal width at which to wrap help messages.

	Using `0` will ignore terminal widths and use source formatting.

	Defaults to current terminal width when `wrap_help` feature flag is enabled. If the flag is disabled or it cannot be determined, the default is 100.

	**NOTE:** This setting applies globally and *not* on a per-command basis.
- `trailing_var_arg` ([[#Boolean Values|boolean]]): Specifies that the final positional argument is a “VarArg” and that the program should not attempt to parse any further args.

	The values of the trailing positional argument will contain all args from itself on.

	**NOTE:** The final positional argument **must** have [[#Arguments]] `multiple_values: true` or the usage string equivalent.
- `version` (string): Sets the version for the short version (`-V`) and help messages.

	If `long_version` is not specified, this message will be displayed for `--version`.
- `visible_alias` (string): Sets a visible alias to this subcommand.

	This allows the subcommand to be accessed via *either* the original name or the given alias. This is more efficient and easier than creating hidden subcommands as one only needs to check for the existence of this command and not all aliased variants.

	**NOTE:** The alias defined with this method is *visible* from the help message and displayed as if it were just another regular subcommand. If looking for an alias that will not be displayed in the help message, see `alias`.
- `visible_long_flag_alias` (string): Add an alias, which functions as a “visible” long flag subcommand.

	This will automatically dispatch as if this subcommand was used. This is more efficient, and easier than creating multiple hidden subcommands as one only needs to check for the existence of this command, and not all variants.
- `visible_short_flag_alias` (single character): Add an alias, which functions as “visible” short flag subcommand

	This will automatically dispatch as if this subcommand was used. This is more efficient, and easier than creating multiple hidden subcommands as one only needs to check for the existence of this command, and not all variants.


### Arguments

Arguments are specified with a mapping named **arg** to a mapping of settings, like this:

```yaml
arg: {
	id: <val>,
	short: <val>,
	long: <val>,
	# ...
}
```

The valid set of settings are as follows:

- `id` ([[#Boolean Values|boolean]]): A unique identifier for the arguement.
- `alias` (string): Fuctions as a hidden long flag.
- `allow_hyphen_values` ([[#Boolean Values|boolean]]): Allows values with start with a leading hyphen (`-`).
- `allow_invalid_utf8` ([[#Boolean Values|boolean]]): The value is allowed to be invalid UTF8.
- `conflicts_with` (string): The `id` of another argument that this arguement conflict with.
- `default_missing_value` (string): Value when the flag is present, but no value is specified.
- `default_value` (string): Value when not present.
- `display_order` (positive number): Allows custom ordering of the args within the help message.
- `env` (string): Read from the given environment variable when arg is not present.
- `exclusive` ([[#Boolean Values|boolean]]): This arg must be passed alone; it conflicts with all other arguments.
- `forbid_empty_values` ([[#Boolean Values|boolean]]): Don't allow an arg to accept explicitly empty values.
- `global` ([[#Boolean Values|boolean]]): Specifies that an arg can be matched to all child subcommands.
- `group` (string): The `id` of the ArgGroup that the arg belongs to.
- `help` (string): The help string for the argument.
- `hide` ([[#Boolean Values|boolean]]): Do not display the arg in help message.
- `hide_default_value` ([[#Boolean Values|boolean]]): Do not display the default value of the arg in the help message.
- `hide_env` ([[#Boolean Values|boolean]]): Do not display the environment variable name in help.
- `hide_env_values` ([[#Boolean Values|boolean]]): Do not display in help any values inside the associated ENV variables for the arg. This is useful when ENV vars contain sensitive values.
- `hide_long_help` ([[#Boolean Values|boolean]]): Hides an arg from long help.
- `hide_possible_values` ([[#Boolean Values|boolean]]): Do not display the possible values in the help message.
- `hide_short_help` ([[#Boolean Values|boolean]]): Hides an arg from short help.
- `ignore_case` ([[#Boolean Values|boolean]]): match values against possible values without matching case.
- `index` (positive number): The index of a positional arguement **starting at** 1.
- `last` ([[#Boolean Values|boolean]]): This arg is the last, or final, positional arg (i.e. has the highest index) and is *only* able to be accessed via the `--` syntax (i.e. `$ prog args -- last_arg`).
- `long` (string): The long version of the argument name (without the `--` characters before it).
- `long_help` (string): The help string for long help.
- `max_occurrences` (positive number): The *maximum* number of occurrences for this arg.
- `max_values` (positive number): The *maximum* number of values for this arg.
- `min_values` (positive number): The *minimum* number of values for this arg.
- `multiple_occurrences` ([[#Boolean Values|boolean]]): Specifies that the arg may appear more than once.
- `multiple_values` ([[#Boolean Values|boolean]]): Specifies that the arg may have an unknown number of values.
- `next_line_help` ([[#Boolean Values|boolean]]): Render the help on the line after the arg.
- `number_of_values` (positive number): The number of values allowed for this arg.
- `overrides_with` (string): Sets an overridable arg. i.e. this arg and the following arg will override each other in POSIX style (whichever arg was specified at runtime **last** "wins").
- `possible_value` (string): Add a possible value for this arg.
- `raw` ([[#Boolean Values|boolean]]): Consume all following args. Do not parse them individually, but rather pass them in entirety. It is worth noting that setting this requires all values to come after a `--` to indicate they should all be captured. For example: `--foo something -- -v -v -v -b -b -b --baz -q -u -x`. This will result in everything after `--` being considered one raw arg.
- `require_equals` ([[#Boolean Values|boolean]]): Requires that options use the `--option=val` syntax (i.e. an equals between the option and the associated value). **NOTE:** Setting this requires `takes_value`.
- `require_value_delimiter` ([[#Boolean Values|boolean]]): Specifies that *multiple values* may only be set using the delimiter.
- `required` ([[#Boolean Values|boolean]]): Specifies that the arg must be present.
- `required_unless_present` (string): Set this arg as **required** as long as the specified arg is not present at runtime.
- `requires` (string): Sets an arg that is required when this one is present (i.e. when using this arg, the following arg *must* be present).
- `short` (single character): The short version of the argument name (without the `-` character before it).
- `short_alias` (single character): Add an alias, which functions as a hidden short flag.
- `takes_value` ([[#Boolean Values|boolean]]): Indicates whether or not this argument takes a value.
- `use_value_delimiter` ([[#Boolean Values|boolean]]): Specifies that an arg should allow grouping of multiple values vis a delimiter.
- `value_delimiter` (single character): Separator between the arg's values. Defaults to `,` (comma).
- `value_name` (string): The string that will appear in the help message for the argument's value.
- `value_terminator` (string): Sentinel to **stop** parsing multiple values of a given arg.
- `visable_alias` (string): Add an alias, which functions as a visible long flag.
- `visible_short_alias` (single character): Add an alias, which functions as a visible short flag.


### Argument Groups

Argument groups are specified with a mapping named **group** to a mapping of settings, like this:

```yaml
group: {
	id: <val>,
	arg: <val>,
	# ...
}
```

The valid set of settings are as follows:

- `id` (string): Sets the group name.
- `arg` ([[#Arguments]]): Adds an [[#Arguments|arg]] to this group by `id`.
- `conflicts_with` (string): Specify an argument or group that must **not** be present when this group is.

	Exclusion (aka conflict) rules function just like arg exclusion rules. You can name other arguments or groups that must *not* be present when one of the arguments from this group are used.

	**NOTE:** The name provided may be an argument or group name.
- `multiple` ([[#Boolean Values|boolean]]): Allows more than one of the args in this group to be used. (Default: `false`)
- `required` ([[#Boolean Values|boolean]]): Require an argument from the group to be present when parsing.

	This is unless conflicting with another argument. A required group will be displayed in the usage string of the application in the format `<arg|arg2|arg3>`.

	**NOTE:** This setting only applies to the current command/subcommands, and not globally.

	**NOTE:** By default, `multiple` is set to `false` which when combined with `required: true` states, “One and *only one* arg must be used from this group. Use of more than one arg is an error.” Advise setting `multiple: true` which states, ’*At least* one arg from this group must be used. Using multiple is OK.“
- `requires` (string): Specify an argument or group that must be present when this group is.

	This is not to be confused with `required`. Requirement rules function just like arg requirement rules. You can name other arguments or groups that must be present when any one of the arguments from this group is used.

	**NOTE:** The name provided may be an argument or group name.