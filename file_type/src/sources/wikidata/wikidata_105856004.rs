use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856004: FileFormat = FileFormat {
    id: 105_856_004,
    source_type: SourceType::Wikidata,
    name: "DeskMate clipart",
    extensions: &["clp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x10, 0x43, 0x4C, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
