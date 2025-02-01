use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849962: FileFormat = FileFormat {
    id: 105_849_962,
    puid: "wikidata/105849962",
    name: "Quattro Pro Clip art",
    extensions: &["clp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x52, 0x54, 0x4C, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
