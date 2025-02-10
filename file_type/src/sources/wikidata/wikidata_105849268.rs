use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849268: FileFormat = FileFormat {
    id: 105_849_268,
    source_type: SourceType::Wikidata,
    name: "GameMaker Studio Project",
    extensions: &["yyp"],
    media_types: &["text/json"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
