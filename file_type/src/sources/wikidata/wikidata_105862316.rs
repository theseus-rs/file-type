use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862316: FileFormat = FileFormat {
    id: 105_862_316,
    puid: "wikidata/105862316",
    name: "GMOD format module",
    extensions: &["gmod"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4D, 0x4F, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
