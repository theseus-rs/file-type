use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857942: FileFormat = FileFormat {
    id: 105_857_942,
    puid: "wikidata/105857942",
    name: "Infinity Engine region/map (v1.x)",
    extensions: &["wed"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x45, 0x44, 0x20, 0x56, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
