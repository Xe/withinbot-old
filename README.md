# withinbot

[![built with
nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)
![Nix](https://github.com/Xe/withinbot/workflows/Nix/badge.svg) ![Written in
Rust](https://img.shields.io/badge/Written%20in-Rust-orange)

A chatbot for The Sub-Aesthetic Furryhole. It has a few commands and features
that help with the guild.

To invoke the bot, either mention it with a command by name or use its prefix
`~`.

This 

## e621 Commands

These commands only run in NSFW channels, for obvious and sanity reasons.

### `get_post <e621_id>`

This fetches information about a given e621 post by ID.

### `search <tag> [tags [tags ...]]`

This performs a search for any number of tags and prints the top 3 results.

## Message Scrapers

This bot also scrapes e621 image links in NSFW chatrooms and links their
correlating e621 post, if it can be found.

## Building

Withinbot is built with [Nix](https://builtwithnix.org). To build it for
production:

```console
$ nix-build
```

To build in a dev shell you can do one of two things. You can install
[lorri](https://github.com/target/lorri) and run:

```console
$ lorri shell
```

Or just run:

```console
$ nix-shell
```
## Configuration

Withinbot requires the following configuration:

| Environment Variable | Description                                          |
| :------------------- | :--------------------------------------------------- |
| `DISCORD_TOKEN`      | A Discord bot token.                                 |
| `MI_TOKEN`           | A token to interface with https://mi.within.website. |

