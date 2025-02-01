use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860848: FileFormat = FileFormat {
    id: 105_860_848,
    puid: "wikidata/105860848",
    name: "Reflex 2 Database",
    extensions: &["r2d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x52, 0x65, 0x66, 0x6C, 0x65, 0x78, 0x32, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
