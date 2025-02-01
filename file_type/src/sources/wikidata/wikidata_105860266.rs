use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860266: FileFormat = FileFormat {
    id: 105_860_266,
    puid: "wikidata/105860266",
    name: "SHELX output",
    extensions: &["res"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x54, 0x4C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
