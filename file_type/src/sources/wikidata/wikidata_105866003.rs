use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866003: FileFormat = FileFormat {
    id: 105_866_003,
    puid: "wikidata/105866003",
    name: "CineMorph Project",
    extensions: &["project"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4F, 0x52, 0x46, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
