use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856782: FileFormat = FileFormat {
    id: 105_856_782,
    puid: "wikidata/105856782",
    name: "Game data package",
    extensions: &["gbx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x42, 0x58, 0x06])],
            },
        }],
    }],
    related_formats: &[],
};
