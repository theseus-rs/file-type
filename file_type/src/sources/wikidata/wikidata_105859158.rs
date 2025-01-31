use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859158: FileFormat = FileFormat {
    id: 105_859_158,
    puid: "wikidata/105859158",
    name: "Lemmings Revolution game data archive",
    extensions: &["box"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x45, 0x4D, 0x42, 0x4F, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
