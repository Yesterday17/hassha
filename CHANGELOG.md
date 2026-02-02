# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-02

### Added

- Initial release of hassha
- Support for 5 Claude Code hook events: `Stop`, `Notification`, `SessionStart`, `SessionEnd`, `PostToolUse`
- 30 predefined JR Yamanote Line departure melodies with `JY-{Station}` naming
- Hierarchical configuration via `.hassha/config.toml`
- Audio file caching in `~/.hassha/audio/`
- CLI commands: `list`, `play`, `cache info`, `cache clear`, `cache prefetch`
- Tool name matching for `PostToolUse` events
- Volume control per hook (0.0 - 1.0)
- macOS audio playback via `afplay`

### Melodies

All 30 stations on the JR Yamanote Line are supported:

- `JY-Tokyo` - SH-3
- `JY-Kanda` - Seseragi
- `JY-Akihabara` - Ogawa V1
- `JY-Okachimachi` - Haru Tremolo
- `JY-Ueno` - Bell B
- `JY-Uguisudani` - Haru Tremolo
- `JY-Nippori` - Haru Tremolo
- `JY-NishiNippori` - Haru Tremolo
- `JY-Tabata` - Haru Tremolo
- `JY-Komagome` - Sakura B
- `JY-Sugamo` - Haru
- `JY-Otsuka` - Haru
- `JY-Ikebukuro` - Melody
- `JY-Mejiro` - Haru
- `JY-Takadanobaba` - Astro Boy
- `JY-ShinOkubo` - Bell B
- `JY-Shinjuku` - Aratana
- `JY-Yoyogi` - Haru
- `JY-Harajuku` - Harajuku A
- `JY-Shibuya` - Hana no Horokobi
- `JY-Ebisu` - Third Man
- `JY-Meguro` - Water Crown
- `JY-Gotanda` - SH-23
- `JY-Osaki` - Umi no Eki
- `JY-Shinagawa` - Seseragi
- `JY-TakanawaGateway` - Sweet Call
- `JY-Tamachi` - Seseragi
- `JY-Hamamatsucho` - Seseragi
- `JY-Shimbashi` - Gota del Vient
- `JY-Yurakucho` - SH-21
