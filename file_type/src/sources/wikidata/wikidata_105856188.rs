use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856188: FileFormat = FileFormat {
    id: 105_856_188,
    source_type: SourceType::Wikidata,
    name: "Device Firmare Upgrade format (generic)",
    extensions: &["dfu"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x52, 0x2D, 0x64, 0x66, 0x75])],
            },
        }],
    }],
    related_formats: &[],
};
