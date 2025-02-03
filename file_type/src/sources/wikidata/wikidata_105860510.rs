use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860510: FileFormat = FileFormat {
    id: 105_860_510,
    source_type: SourceType::Wikidata,
    name: "Borland Reflex 2 color settings",
    extensions: &["r2z"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x65, 0x66, 0x6C, 0x65, 0x78, 0x20, 0x32, 0x00, 0x43, 0x6F, 0x6C, 0x6F,
                    0x72, 0x20, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x00, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
