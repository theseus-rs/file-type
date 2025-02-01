use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851728: FileFormat = FileFormat {
    id: 105_851_728,
    puid: "wikidata/105851728",
    name: "Superbase form definition",
    extensions: &["sbv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x44, 0x46, 0x4F, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
