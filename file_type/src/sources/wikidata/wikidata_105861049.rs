use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861049: FileFormat = FileFormat {
    id: 105_861_049,
    puid: "wikidata/105861049",
    name: "MegaPaint symbols Library",
    extensions: &["lib"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x4C, 0x49, 0x42, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
