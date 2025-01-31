use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113201649: FileFormat = FileFormat {
    id: 113_201_649,
    puid: "wikidata/113201649",
    name: "LiveMotion Project File, version 2",
    extensions: &["liv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0xE6, 0xFF, 0xFF]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x47, 0x5A, 0x20, 0x32, 0x2E]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
