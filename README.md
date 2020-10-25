# utf8tool

A simple command-line tool that accepts Unicode codepoint ranges as input, and produces an UTF-8 file containing all characters contained in these ranges, along with hopefully useful annotations.  
The tool is also able to produce a sorted list of all characters contained in a given set of files.

This is mainly useful for preparing a list of characters to feed to a font packer such as [Hiero](https://github.com/libgdx/libgdx/wiki/Hiero) or [gdx-fontpack](https://github.com/mattdesl/gdx-fontpack). It can also assist in determining which codepoint ranges your application actually needs, along with how many ranges and characters are actually used.

All files read by the tool are assumed to be UTF-8 encoded. Output files are always UTF-8 encoded.

# In this repository

The `results` directory contains some output files. This might actually be the only thing you need.  
The `sample_scripts` directory contains input files for testing purposes.  
The `sample_texts` directory contains a few UTF-8 text files that are used in the tutorial.  

# Tutorial

## Basic usage

First of all, the tool accepts a single input file as its first command-line argument. This means that if you're on Windows, you can simply drag-and-drop the file onto the executable, to run it without having to open a command prompt.

Given the following input file (or "script"):

```text
output_file_path: basic_japanese_list.txt
U+3040 ... U+309F: Hiragana
U+30A0 ... U+30FF: Katakana
```

An output UTF-8 file named `basic_japanese_list.txt` (as requested) is produced with the following contents:

```text
# Range: U+3040 ... U+309F (96 chars)
# Name: Hiragana
぀ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖ゗゘゙゚゛゜ゝゞゟ

# Range: U+30A0 ... U+30FF (96 chars)
# Name: Katakana
゠ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶヷヸヹヺ・ーヽヾヿ

# Overall stats: 2 ranges, 192 chars
```

What we did was simply include a list of Unicode ranges (`U+XXXX ... U+XXXX`) and give them a name (the name has no purpose, other than clarity).  
As a result, we obtained an UTF-8 file containing all characters in the requested ranges; we can now easily select each line and copy it anywhere we like.

## Commands showcase: disabling annotations

The tool actually interprets the input file as an ordered list of commands, which allows you to customize the output and perform various related tasks.

For instance, you might want to remove all annotations from the output, in order to produce a single file that "glues" all characters together; this is easily done as follows:

```text
# By the way, when a line starts with '#', the line is ignored by the tool, allowing you to add comments.
output_file_path: basic_japanese_list_without_annotations.txt

# The "range suffix" is a string appended to the list of chars in a range. By default it is "\n\n" (two line-feeds), but by setting "none" we explicitly disable it.
range_suffix: none

# Disable the annotations preceding each list of chars.
range_headers: none

# Disable the "Overall stats" annotation at the end.
overall_stats: none

# Then request the following ranges.
U+3040 ... U+309F: Hiragana
U+30A0 ... U+30FF: Katakana
```

The output file, `basic_japanese_list_without_annotations.txt` contains:

```
぀ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖ゗゘゙゚゛゜ゝゞゟ゠ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶヷヸヹヺ・ーヽヾヿ
```

## Commands showcase: restricting which characters to include ("filter" feature)

Generally, when preparing a font for a game, you can simply select the complete set of characters used by whichever languages you intend to support.

However, for languages that require many characters (such as Chinese), it is probably overkill to include all of these characters in your game, especially since it is likely that you only actually need a small subset of these.

If you tried including all characters from the range `U+20000 ... U+2A6DF: CJK Unified Ideographs Extension B` (for instance), which represents no less than 42720 characters, you _might_ encounter memory issues, especially if you are baking those characters into texture atlases, with multiple fonts and arbitrarily large font sizes.

In such cases, it is probably more efficient to only include the characters that your game actually needs.

In all likelihood, all of the localized strings required for your game are present in arbitrary data files. If your strings are available in UTF-8 text files, then this tool can produce a sorted list of all of the characters that are present in these files.

The feature is called "filter" because the resulting set of characters is used to restrict which characters are emitted for the requested ranges.

A simple example is as follows:

```
output_file_path: filter_test_filtered_charlist.txt

# The `filter_add_files` adds the set of characters of each given file into the current "filter", which is itself a set of characters.
# You can notice that traditional BASH-style wildcard expansion (*, **, ?) is supported, for easily adding many files which paths follow a common pattern.
filter_add_files: lorem_ipsum_*.txt Cargo.toml

# The "filter" now includes all characters contained in "Cargo.toml", and also those from all files in the current directory which names start with "lorem_ipsum_" and end with ".txt".
# The `filter_dump` command allows writing this set of characters into a single output file.
filter_dump: filter_test_complete_charlist.txt

# We can request character ranges as usual; this time, only the characters present in the "filter" will be written to the output file.
U+0020 ... U+007F: Basic Latin
U+3040 ... U+309F: Hiragana
U+30A0 ... U+30FF: Katakana
```

We also make sure to prepare the following file, `lorem_ipsum_la.txt`:

```
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

... As well as `lorem_ipsum_ja.txt`:

```
待へみはよ太4主チホ広六領レマ氏一わくばす読浦てげ土情スコツ開生めら変辺ろ感北コケツ伸1力マ盛側ゃとごぼ天樫国仕ラ件喚宴嶺るねぴ。岡くめルぐ供右セリニ当敬レお戦義トづ田開おりい関37技ぐざつ職雪姿フユ圧付ほでに余的ヤキエ暮確ばしのぜ言除ッ最建マサヱ季広エツ慶月チ米海ぐせてル無辺堪薫ょねで。大ミスワ整売いな障海公結意ノミヤナ住町特かぴぐ会87寄テミイネ材山むず白料力山そ。
```

When executing the tool with the script given as example, we get two output files. The first one, `filtered_test_complete_charlist.txt`, is simply the set of all characters used in the files given to the `filter_add_files` command; we requested it via the `filter_dump` command.

```

 "#,-./0123478:<=>@DELSU[]abcdefghiklmnopqrstuvxy。いおかくぐげござしすずせぜそつづてでとなにねのはばぴへほぼみむめゃょよらりるろわイエキケコサスセチッツテトナニネノフホマミヤユラリルレワヱ一主仕付件会伸住余供側公六力北右喚国土圧堪売変大天太姿季宴寄山岡嶺広建当待情意感慶戦技敬整料暮最月材樫氏浦海無特生田町白的盛確米結義職薫言読辺開関除障雪領
```

We can notice that the file starts with a line feed; this is because at least one of the files had a line feed (`\n`) in it, so this counts as a character to include; this is also true for other control characters such as `\r`. This is by design and you can easily get rid of them manually, since they will always appear before any "printable" character.

Let's take a look at the second output file, `filtered_test_filtered_charlist.txt`:

```
# Range: U+0020 ... U+007F (49 chars)
# Name: Basic Latin
 "#,-./0123478:<=>@DELSU[]abcdefghiklmnopqrstuvxy

# Range: U+3040 ... U+309F (40 chars)
# Name: Hiragana
いおかくぐげござしすずせぜそつづてでとなにねのはばぴへほぼみむめゃょよらりるろわ

# Range: U+30A0 ... U+30FF (29 chars)
# Name: Katakana
イエキケコサスセチッツテトナニネノフホマミヤユラリルレワヱ

# Overall stats: 3 ranges, 118 chars
```

This file lists all characters contained in the requested ranges, **except** those that were not present in the files given to the `filter_add_files` command.  

Hopefully you can see how much control this gives you; you can easily dump all characters into a font packer, or you can cherry-pick from each range, if you wish to use different texture atlases for each language.

## Wait, it's all script?

Always has been.

```
output_file_path: fun.txt
U+1F000 ... U+1F02F: Mahjong Tiles
U+1F030 ... U+1F09F: Domino Tiles
U+1F0A0 ... U+1F0FF: Playing Cards

filter_add_files: lorem_ipsum_*.txt Cargo.toml fun.txt
output_file_path: basic_latin_and_jap.txt
U+0020 ... U+007F: Basic Latin
U+3040 ... U+309F: Hiragana
U+30A0 ... U+30FF: Katakana

filter_clear:
filter_add_files: README.md
output_file_path: fun.txt
U+0020 ... U+007F: Basic Latin
```

You can clear the current filter via the `filter_clear` command.  
You can use `output_file_path` multiple times, because it simply sets the current file to write to. When done multiple times with a same target file, the target file is only truncated the first time it is encountered.

You may not even need to request ranges at all!

```
filter_add_files: game/texts/**.txt
filter_dump: all_chars_in_the_game.txt
```
