use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43989331: FileFormat = FileFormat {
    id: 43_989_331,
    puid: "wikidata/43989331",
    name: "Quicken Data Format",
    extensions: &["qdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xAC, 0x9E, 0xBD, 0x8F, 0x00, 0x00]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
