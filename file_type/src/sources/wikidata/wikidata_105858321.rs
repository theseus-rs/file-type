use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858321: FileFormat = FileFormat {
    id: 105_858_321,
    puid: "wikidata/105858321",
    name: "Efficient and Easy to use Binary Format",
    extensions: &["ebf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x8A, 0x45, 0x42, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
