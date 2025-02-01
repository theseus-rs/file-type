use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858594: FileFormat = FileFormat {
    id: 105_858_594,
    puid: "wikidata/105858594",
    name: "Casio QV digital CAMera bitmap",
    extensions: &["cam"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x20, 0x4D, 0x4D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
