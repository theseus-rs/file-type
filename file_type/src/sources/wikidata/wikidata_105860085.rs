use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860085: FileFormat = FileFormat {
    id: 105_860_085,
    puid: "wikidata/105860085",
    name: "Delphine CIN video",
    extensions: &["cin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0xAA, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
