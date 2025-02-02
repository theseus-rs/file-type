use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856387: FileFormat = FileFormat {
    id: 105_856_387,
    source_type: SourceType::Wikidata,
    name: "DoubleSpace compressed volume (v6.0)",
    extensions: &["001"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEB, 0x3C, 0x90, 0x4D, 0x53, 0x44, 0x53, 0x50, 0x36, 0x2E, 0x30, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
