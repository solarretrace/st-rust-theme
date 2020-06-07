
# Design ideas


If some context is expected (even if optional), push it. The context should pop itself.


    <key>gutterSettings</key><dict>
        <key>background</key><string>#2b2b2b</string>
        <key>divider</key><string>#3b3b3b</string>
        <key>foreground</key><string>#676650</string>
        <key>selectionBackground</key><string>#444444</string>
        <key>selectionForeground</key><string>#8ba596</string>
    </dict>

    <key>settings</key><dict>
        <key>background</key><string>#252532</string>
        <key>caret</key><string>#cee2e3</string>
        <key>foreground</key><string>#d0d0d0</string>
        <key>invisibles</key><string>#676650</string>
        <key>lineHighlight</key><string>#3b3b3b</string>
        <key>selection</key><string>#770077</string>
    </dict>

keyword.operator                #a0a0a0 White-grey
markup.bold                     #a0a0a0 
punctuation                     #a0a0a0 
constant.character.escape       #ab1234 Red
meta.selector                   #ff9900 Purple
constant                        #c82500 Yellow
string                          #ab4567 D. Magenta
support.function                #47c266 L. Green
entity.name.function            #4fa0df Blue
variable                        #8a71a2 L. Purple
keyword                         #dd7700 Orange
keyword.operator                #c0c0c0 Grey
comment                         #559970 Green, Italic
variable.parameter.function     #cee2e3 White "Text"?



keyword.rust
punctuation.definition.group.rust

# List of scopes in use
comment.block.documentation.inner.rust
comment.block.documentation.outer.rust
comment.block.rust
comment.line.documentation.inner.rust
comment.line.documentation.outer.rust
comment.line.rust
comment.tag.rust
constant.character.escape.rust
constant.language.rust
constant.numeric.float.rust
constant.numeric.integer.binary.rust
constant.numeric.integer.decimal.rust
constant.numeric.integer.hexadecimal.rust
constant.numeric.integer.octal.rust
constant.other.placeholder.rust
entity.name.path.rust
invalid.illegal.byte.rust
invalid.illegal.char.rust
invalid.illegal.character.escape.rust
invalid.illegal.character.numeric.rust
invalid.illegal.keyword.rust
keyword.control.conditional.rust
keyword.control.rust
keyword.operator.arithmetic.rust
keyword.operator.assignment.rust
keyword.operator.boolean.rust
keyword.operator.logical.rust
keyword.operator.rust
keyword.operator.string-prefix.rust
keyword.operator.word.rust
keyword.other.rust
keyword.rust
keyword.path.rust
meta.path.rust
punctuation.accessor.path.rust
punctuation.accessor.rust
punctuation.definition.comment.begin.rust
punctuation.definition.comment.documentation.inner.begin.rust
punctuation.definition.comment.documentation.inner.end.rust
punctuation.definition.comment.documentation.inner.rust
punctuation.definition.comment.documentation.outer.begin.rust
punctuation.definition.comment.documentation.outer.end.rust
punctuation.definition.comment.documentation.outer.rust
punctuation.definition.comment.end.rust
punctuation.definition.comment.rust
punctuation.definition.group.begin.rust
punctuation.definition.group.end.rust
punctuation.definition.string.begin.rust
punctuation.definition.string.end.rust
punctuation.other.rust
punctuation.rust
punctuation.separator.continuation.rust
punctuation.separator.rust
punctuation.terminator.rust
storage.type.rust
string.quoted.double.raw.rust
string.quoted.double.rust
string.quoted.single.rust


# Minimal syntax highlighting scopes
comment
constant
constant.character.escape
constant.language
constant.numeric
entity.name
entity.name.section
entity.name.tag
entity.other.attribute-name
entity.other.inherited-class
invalid
invalid.deprecated
keyword
keyword.control
keyword.declaration
keyword.operator
storage.modifier
storage.type
string
support
variable
variable.function
variable.language
variable.parameter

abstract|become|box|do|final|macro|override|priv|try|typeof|unsized|virtual|yield

https://www.sublimetext.com/docs/3/scope_naming.html

# comment.
comment.line
comment.block
comment.block.documentation
punctuation.definition.comment
meta.toc-list



# constant.
constant.language
constant.character.escape
constant.other.placeholder

constant.other
constant.numeric
constant.numeric.integer
constant.numeric.integer.binary
constant.numeric.integer.octal
constant.numeric.integer.decimal
constant.numeric.integer.hexadecimal
constant.numeric.integer.other
constant.numeric.float
constant.numeric.float.binary
constant.numeric.float.octal
constant.numeric.float.decimal
constant.numeric.float.hexadecimal
constant.numeric.float.other
constant.numeric.complex
constant.numeric.complex.real
constant.numeric.complex.imaginary


# entity.
entity.name.class
entity.name.struct
entity.name.enum
entity.name.union
entity.name.trait
entity.name.interface
entity.name.impl
entity.name.type

entity.name.function
entity.name.function.constructor
entity.name.function.destructor


# invalid.
invalid.illegal

# keyword.
keyword.control
keyword.control.conditional
keyword.control.import

keyword.operator
keyword.operator.assignment
keyword.operator.arithmetic
keyword.operator.bitwise
keyword.operator.logical
keyword.operator.word
keyword.operator.other


# markup.
markup.heading

markup.list.unnumbered
markup.list.numbered

markup.bold
markup.underline
markup.underline.link
markup.italic
markup.inserted
markup.deleted
markup.quote
markup.raw.inline
markup.raw.block
markup.other



# meta.
meta.string


# punctuation.
punctuation.seperator
punctuation.seperator.continuation
punctuation.terminator
punctuation.accessor


# source.
source

# storage.
storage.type
storage.modifier

storage.type.function 
keyword.declaration.function
storage.type.class
keyword.declaration.class
storage.type.struct
keyword.declaration.struct
storage.type.enum
keyword.declaration.enum
storage.type.union
keyword.declaration.union
storage.type.trait
keyword.declaration.trait
storage.type.interface
keyword.declaration.interface
storage.type.impl
keyword.declaration.impl
storage.type
keyword.declaration.type


# string.
meta.string

string.quoted.single
string.quoted.double
string.quoted.triple
string.quoted.other
string.unquoted
string.regexp

punctuation.definition.string.begin
punctuation.definition.string.end

meta.interpolation
punctuation.section.interpolation.begin
punctuation.section.interpolation.end
source.language-suffix.embedded


# support.

support.constant
support.function
support.module
support.type
support.class

# text.
# variable.

