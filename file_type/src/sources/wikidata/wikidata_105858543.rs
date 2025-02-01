use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858543: FileFormat = FileFormat {
    id: 105_858_543,
    puid: "wikidata/105858543",
    name: "MalieGF bitmap",
    extensions: &["mgf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x61, 0x6C, 0x69, 0x65, 0x47, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
