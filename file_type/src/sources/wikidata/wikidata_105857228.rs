use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857228: FileFormat = FileFormat {
    id: 105_857_228,
    source_type: SourceType::Wikidata,
    name: "HEC-HMS Basin model settings",
    extensions: &["basin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x61, 0x73, 0x69, 0x6E, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
