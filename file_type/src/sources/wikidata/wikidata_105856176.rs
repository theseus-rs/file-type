use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856176: FileFormat = FileFormat {
    id: 105_856_176,
    source_type: SourceType::Wikidata,
    name: "Darwin Pond File",
    extensions: &["dwp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2E, 0x44, 0x57, 0x50, 0x03, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
