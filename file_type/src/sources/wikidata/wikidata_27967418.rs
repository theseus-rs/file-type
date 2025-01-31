use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967418: FileFormat = FileFormat {
    id: 27_967_418,
    puid: "wikidata/27967418",
    name: "GYM",
    extensions: &["gym"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x59, 0x4D, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
