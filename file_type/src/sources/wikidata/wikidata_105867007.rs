use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867007: FileFormat = FileFormat {
    id: 105_867_007,
    puid: "wikidata/105867007",
    name: "Rumble Fighter game data archive",
    extensions: &["nsz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x53, 0x5A, 0x6A])],
            },
        }],
    }],
    related_formats: &[],
};
