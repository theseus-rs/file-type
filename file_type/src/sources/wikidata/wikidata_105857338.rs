use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857338: FileFormat = FileFormat {
    id: 105_857_338,
    source_type: SourceType::Wikidata,
    name: "Firefox bookmark (JavaScript Object Notation)",
    extensions: &["json"],
    media_types: &["text/json"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
