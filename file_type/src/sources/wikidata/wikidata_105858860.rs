use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858860: FileFormat = FileFormat {
    id: 105_858_860,
    source_type: SourceType::Wikidata,
    name: "Sony PS3 Silk Web Browser container format",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x49, 0x4C, 0x4B, 0x50, 0x41, 0x44, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
