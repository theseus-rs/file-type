use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861525: FileFormat = FileFormat {
    id: 105_861_525,
    puid: "wikidata/105861525",
    name: "LIFE 3000 status",
    extensions: &["lif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x49, 0x46, 0x45, 0x20, 0x33, 0x30, 0x30, 0x30, 0x00, 0x56, 0x31, 0x2E,
                    0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
