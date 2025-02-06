use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858434: FileFormat = FileFormat {
    id: 105_858_434,
    source_type: SourceType::Wikidata,
    name: "Easy Grade Pro data",
    extensions: &["egp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x42, 0x48, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
