use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856075: FileFormat = FileFormat {
    id: 105_856_075,
    source_type: SourceType::Wikidata,
    name: "Full Tilt! Pinball table data",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x52, 0x54, 0x4F, 0x55, 0x54, 0x28, 0x34, 0x2E, 0x30, 0x29, 0x52,
                    0x45, 0x53, 0x4F, 0x55, 0x52, 0x43, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
