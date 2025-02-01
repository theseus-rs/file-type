use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862553: FileFormat = FileFormat {
    id: 105_862_553,
    puid: "wikidata/105862553",
    name: "MegaStation module",
    extensions: &["ms"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4F, 0x4E, 0x47, 0x30, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
