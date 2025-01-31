use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860291: FileFormat = FileFormat {
    id: 105_860_291,
    puid: "wikidata/105860291",
    name: "M.A.X. game data archive",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x45, 0x53, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
