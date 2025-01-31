use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51751659: FileFormat = FileFormat {
    id: 51_751_659,
    puid: "wikidata/51751659",
    name: "Harvard Graphics Show, version 3",
    extensions: &["sh3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x47, 0x42, 0x31, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
