---
description: Accepts `log` events and allows you to coerce log fields into fixed types.
---

<!--
     THIS FILE IS AUTOGENERATED!

     To make changes please edit the template located at:

     scripts/generate/templates/docs/usage/configuration/transforms/coercer.md.erb
-->

# coercer transform

![][assets.coercer_transform]


The `coercer` transform accepts [`log`][docs.data-model.log] events and allows you to coerce log fields into fixed types.

## Config File

{% code-tabs %}
{% code-tabs-item title="vector.toml (simple)" %}
```coffeescript
[transforms.my_transform_id]
  type = "coercer" # must be: "coercer"
  inputs = ["my-source-id"]

  # For a complete list of options see the "advanced" tab above.
```
{% endcode-tabs-item %}
{% code-tabs-item title="vector.toml (advanced)" %}
```coffeescript
[transforms.coercer_transform]
  #
  # General
  #

  # The component type
  # 
  # * required
  # * no default
  # * must be: "coercer"
  type = "coercer"

  # A list of upstream source or transform IDs. See Config Composition for more
  # info.
  # 
  # * required
  # * no default
  inputs = ["my-source-id"]

  #
  # Types
  #

  [transforms.coercer_transform.types]
    # A definition of log field type conversions. They key is the log field name
    # and the value is the type. `strftime` specifiers are supported for the
    # `timestamp` type.
    # 
    # * required
    # * no default
    # * enum: "string", "int", "float", "bool", and "timestamp|strftime"
    status = "int"
    duration = "float"
    success = "bool"
    timestamp = "timestamp|%s"
    timestamp = "timestamp|%+"
    timestamp = "timestamp|%F"
    timestamp = "timestamp|%a %b %e %T %Y"
```
{% endcode-tabs-item %}
{% endcode-tabs %}

## Examples

Given the following input event:

{% code-tabs %}
{% code-tabs-item title="log" %}
```json
{
  // ... existing fields
  "bytes_in": "5667",
  "bytes_out": "20574",
  "host": "5.86.210.12",
  "message": "GET /embrace/supply-chains/dynamic/vertical",
  "status": "201",
  "timestamp": "19/06/2019:17:20:49 -0400",
  "user_id": "zieme4647"
}
```
{% endcode-tabs-item %}
{% endcode-tabs %}

And the following configuration:

{% code-tabs %}
{% code-tabs-item title="vector.toml" %}
```coffeescript
[transforms.<transform-id>]
  type = "coercer"

[transforms.<transform-id>.types]
  bytes_in = "int"
  bytes_out = "int"
  timestamp = "timestamp|%m/%d/%Y:%H:%M:%S %z"
  status = "int"
```
{% endcode-tabs-item %}
{% endcode-tabs %}

A [`log` event][docs.data-model.log] will be emitted with the following structure:

```javascript
{
  // ... existing fields
  "bytes_in": 5667,
  "bytes_out": 20574,
  "host": "5.86.210.12",
  "message": "GET /embrace/supply-chains/dynamic/vertical",
  "status": 201,
  "timestamp": <19/06/2019:17:20:49 -0400>,
  "user_id": "zieme4647"
}
```

## How It Works

### Environment Variables

Environment variables are supported through all of Vector's configuration.
Simply add `${MY_ENV_VAR}` in your Vector configuration file and the variable
will be replaced before being evaluated.

You can learn more in the [Environment Variables][docs.configuration#environment-variables]
section.

### Types

By default, extracted (parsed) fields all contain `string` values. You can
coerce these values into types via the `types` table as shown in the
[Config File](#config-file) example above. For example:

```coffeescript
[transforms.my_transform_id]
  # ...

  # OPTIONAL - Types
  [transforms.my_transform_id.types]
    status = "int"
    duration = "float"
    success = "bool"
    timestamp = "timestamp|%s"
    timestamp = "timestamp|%+"
    timestamp = "timestamp|%F"
    timestamp = "timestamp|%a %b %e %T %Y"
```

The available types are:

| Type        | Desription                                                                                                          |
|:------------|:--------------------------------------------------------------------------------------------------------------------|
| `bool`      | Coerces to a `true`/`false` boolean. The `1`/`0` and `t`/`f` values are also coerced.                               |
| `float`     | Coerce to 64 bit floats.                                                                                            |
| `int`       | Coerce to a 64 bit integer.                                                                                         |
| `string`    | Coerces to a string. Generally not necessary since values are extracted as strings.                                 |
| `timestamp` | Coerces to a Vector timestamp. [`strftime` specificiers][urls.strftime_specifiers] must be used to parse the string. |

## Troubleshooting

The best place to start with troubleshooting is to check the
[Vector logs][docs.monitoring#logs]. This is typically located at
`/var/log/vector.log`, then proceed to follow the
[Troubleshooting Guide][docs.troubleshooting].

If the [Troubleshooting Guide][docs.troubleshooting] does not resolve your
issue, please:

1. Check for any [open `coercer_transform` issues][urls.coercer_transform_issues].
2. If encountered a bug, please [file a bug report][urls.new_coercer_transform_bug].
3. If encountered a missing feature, please [file a feature request][urls.new_coercer_transform_enhancement].
4. If you need help, [join our chat/forum community][urls.vector_chat]. You can post a question and search previous questions.


### Alternatives

Finally, consider the following alternatives:

* [`lua` transform][docs.transforms.lua]

## Resources

* [**Issues**][urls.coercer_transform_issues] - [enhancements][urls.coercer_transform_enhancements] - [bugs][urls.coercer_transform_bugs]
* [**Source code**][urls.coercer_transform_source]


[assets.coercer_transform]: ../../../assets/coercer-transform.svg
[docs.configuration#environment-variables]: ../../../usage/configuration#environment-variables
[docs.data-model.log]: ../../../about/data-model/log.md
[docs.monitoring#logs]: ../../../usage/administration/monitoring.md#logs
[docs.transforms.lua]: ../../../usage/configuration/transforms/lua.md
[docs.troubleshooting]: ../../../usage/guides/troubleshooting.md
[urls.coercer_transform_bugs]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22transform%3A+coercer%22+label%3A%22Type%3A+bug%22
[urls.coercer_transform_enhancements]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22transform%3A+coercer%22+label%3A%22Type%3A+enhancement%22
[urls.coercer_transform_issues]: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22transform%3A+coercer%22
[urls.coercer_transform_source]: https://github.com/timberio/vector/tree/master/src/transforms/coercer.rs
[urls.new_coercer_transform_bug]: https://github.com/timberio/vector/issues/new?labels=transform%3A+coercer&labels=Type%3A+bug
[urls.new_coercer_transform_enhancement]: https://github.com/timberio/vector/issues/new?labels=transform%3A+coercer&labels=Type%3A+enhancement
[urls.strftime_specifiers]: https://docs.rs/chrono/0.3.1/chrono/format/strftime/index.html
[urls.vector_chat]: https://chat.vector.dev
