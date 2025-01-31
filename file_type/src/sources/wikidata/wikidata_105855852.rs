use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855852: FileFormat = FileFormat {
    id: 105_855_852,
    puid: "wikidata/105855852",
    name: "DC2N DMP format (v0)",
    extensions: &["dmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x43, 0x32, 0x4E, 0x2D, 0x54, 0x41, 0x50, 0x2D, 0x52, 0x41, 0x57, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
