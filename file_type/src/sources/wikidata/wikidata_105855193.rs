use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855193: FileFormat = FileFormat {
    id: 105_855_193,
    puid: "wikidata/105855193",
    name: "Turbo Rascal Syntax Error graphic",
    extensions: &["flf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4C, 0x55, 0x46, 0x46, 0x36, 0x34, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
