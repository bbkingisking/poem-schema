# poem.schema.json

Schema specification for `.poem` files: a custom YAML-based format for storing poetry. Mainly made to be used with my [leaves](https://github.com/bbkingisking/leaves) TUI poem reader.

## File Structure

Each `.poem` file is a valid YAML object with one or more version keys. The required top-level key is `canonical`, but others (for example `original_spelling`, `english_translation`, `1798_edition`, `hanja_eum`) may be included.

Each version must include:

- `text` (string)

Optional fields:

- `title` (string)
- `author` (string)
- `language` (string)
- `epigraph` (string)
- `rtl` (boolean)
- `vertical` (boolean)

And any custom fields.

## Examples

Example minimal poem (one version, only required fields):

```yaml
canonical:
  text: my monoline poem
```

Example poem with different versions, languages, and layouts:

```yaml
canonical:
  title: Scene of Snow
  author: Kim Sat Gat
  language: eng
  text: |
    Snow flakes flying down
    Like March butterflies;
    Stepped on,
    Making sounds of June frogs

    The host insists on
    Staying for snow and coldness;
    Wishing his guest to stay drunken,
    Another drink, he offers.

original:
  title: 雪景
  author: 金炳淵
  language: lzh
  rtl: true
  vertical: true
  text: |
    飛來片片三月蝶
    踏去聲聲六月蛙
    寒將不去多言雪
    醉或以留更進盃

korean:
  title: 설경
  author: 김삿갓
  language: kor
  text: |
    펄펄 날리니 춘삼월 나비같고
    뽀드득 밟는 소리 유월 개구리같네
    추워서 못 가신다고 눈을 자꾸 핑계 대며
    취중에 행여 머무를까 다시 술잔을 내미네

hanja_eum:
  title: 설경 (한자음)
  author: 김삿갓
  language: kor
  text: |
    비래편편삼월접
    답거성성유월와
    한장불거다언설
    취혹이류갱진배
```

## Validation

A Rust validator is included in `./validators/rust`. To build it from source, ensure you have [`rustc`](https://rust-lang.org/tools/install/) installed, then, from `./validators/rust/` run `cargo build --release`. To use the binary, pass the full path as a CLI argument, for example: `./validate-poem /Users/bbkingisking/literature/william-blake_the-tyger.poem`. You will get an `[OK]` message if it is valid, or a detailed error message saying why it is not.

> [!NOTE]
> The schema specification allows arbitrary strings for the `language` property. If you want to ensure that your `language` fields are valid [`ISO-639-3`](https://en.wikipedia.org/wiki/ISO_639-3) and [`ISO-15924`](https://en.wikipedia.org/wiki/ISO_15924) tags (for example, `lzh-Hant`), use the `--strict-language` flag. This is an optional feature of the validator and does not relate to the schema itself.

## Why YAML?

For storing and validating poetry, I see two main goals: structural fidelity and human readability. In other words, a format that is able to capture all the content and metadata of a poem while being as readable as possible.

Readability is especially important in this case, as many people who deal with poetry do not necessarily deal with or care about data structures or programming syntax conventions.

YAML is a wild, messy, but fundamentally expressive language that maximizes both goals. Some delightful examples of that are string [plain style](https://yaml.org/spec/1.2.2/#733-plain-style), which allows easy-on-the-eyes unquoted plain text values for properties like titles and authors, [literal style](https://yaml.org/spec/1.2.2/#literal-style) blocks, which allow the input of a poem in plain text and preserve relative indentation, line breaks, and stanza breaks, and the sadly deprecated ["yes" format for booleans](https://yaml.org/spec/1.1/#id864510).

## Why so minimal?

The schema is meant to match any valid poem. Plenty of poetry, for example folk poetry, is anonymous, either by design or as a result of time. Even more common are poems without titles, where editors typically use the first line of the poem as a title for indexing purposes. But at its core, the only strictly necessary property of a poem is the poem itself, that is to say, the `text` property.

## Why so loose?

`additionalProperties` are explicitly allowed at the schema level. That means you can add any custom keys, values, and nested amalgamations of both for your own indexing purposes and still use the schema and the validators to ensure consistency for the core elements. Very useful if you want to piggyback off of the existing validation and add your own custom indexing fields.

## In action

My [collection of poetry](https://github.com/bbkingisking/poetry) follows this schema and uses the Rust validator as part of its GitHub CI.

## Limitations

- Poems in which the first line is indented require additional syntax. See [this poem](https://github.com/bbkingisking/poetry/blob/main/%D0%B2%D0%BE%D0%B9%D0%BD%D0%B0_%D0%B0%D0%BB%D0%B5%D0%BA%D1%81%D0%B0%D0%BD%D0%B4%D1%8A%D1%80-%D0%BF%D1%83%D1%88%D0%BA%D0%B8%D0%BD.poem) for an example or the [YAML specs](https://yaml.org/spec/1.2.2/#8111-block-indentation-indicator) for a formal explanation of block indentation indicators.

- Not clear how suitable the schema is for [visual poetry](https://en.wikipedia.org/wiki/Visual_poetry)
