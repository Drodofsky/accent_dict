# accent-dict.jp
This Anki addon allows you to easily add audio files, accent diagrams (as SVG), and kanji to your vocabulary cards. 
It's based on a fork of the [Monokakido dictionary implementation](https://github.com/golddranks/monokakido) by golddranks. 
All the necessary assets are sourced from the [NHK dictionary](https://www.monokakido.jp/en/android/nhkaccent2/index.html).
Please note that this addon is still in development, and no shippable version is available.

## Notice

This library started as a personal project driven by curiosity.
It is NOT intended to support piracy;
I strongly condemn making unauthorized copies of Monokakido's dictionaries,
and take no part or responsibility in that kind of activity.
Please buy your dictionaries directly from Monokakido to show your love and support.




![Anki](media/anki.gif)

## Supported Platforms

This add-on uses a native (compiled) backend and is therefore platform-dependent.

### Tier 1 (Fully Supported)
- **Linux (x86_64)**  
  Actively developed and tested. This is the primary supported platform.

### Tier 2 (Supported)
- **Windows (x86_64)**  
  Expected to work correctly. Fewer real-world tests than Linux.

### Tier 3 (Experimental)
- **macOS (Apple Silicon & Intel)**  
  Provided on a best-effort basis. Not actively tested.

## Installation

### 1. Install the Add-on

1. Open Anki
2. Go to **Tools → Add-ons**
3. Click **Get Add-ons...**
4. Enter this Add-on ID: 1048189751
5. Restart Anki after installation.

### 2. Manual Installation of Dictionary Files (Required)
1. Open the add-on folder: "/addons21/1048189751/"
2. Create the folder structure: "user_files/assets/"
3. Copy all dictionary files into the `assets` folder

The final folder structure should look as follows:
```
.
.
├── __init__.py
└── user_files
    └── assets
        ├── audio
        │   ├── 00000.nrsc
        │   ├── 00001.nrsc
        │   ├── 00002.nrsc
        │   ├── 00003.nrsc
        │   ├── 00004.nrsc
        │   ├── 00005.nrsc
        │   ├── 00006.nrsc
        │   ├── 00007.nrsc
        │   ├── 00008.nrsc
        │   ├── 00009.nrsc
        │   ├── 00010.nrsc
        │   ├── 00011.nrsc
        │   ├── 00012.nrsc
        │   ├── 00013.nrsc
        │   ├── 00014.nrsc
        │   ├── 00015.nrsc
        │   ├── 00016.nrsc
        │   ├── 00017.nrsc
        │   └── index.nidx
        ├── contents
        │   ├── contents-0001.rsc
        │   ├── contents-0002.rsc
        │   ├── contents-0003.rsc
        │   ├── contents.idx
        │   └── contents.map
        ├── headline
        │   ├── headline.headlinestore
        │   └── short-headline.headlinestore
        ├── key
        │   ├── compound.keyindex
        │   ├── headword.keyindex
        │   └── numeral.keyindex
```
## Credits

- Based on a fork of the Monokakido library by [golddranks](https://github.com/golddranks/monokakido).
- Accent diagram inspired by [SVG_pitch](https://github.com/IllDepence/SVG_pitch).
