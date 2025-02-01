use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29651082: FileFormat = FileFormat {
    id: 29_651_082,
    puid: "wikidata/29651082",
    name: "Portable aRchive eXchange",
    extensions: &["pax"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x5A, 0x46, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
