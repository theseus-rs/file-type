use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866349: FileFormat = FileFormat {
    id: 105_866_349,
    puid: "wikidata/105866349",
    name: "Starbound player data (v1.1)",
    extensions: &["player"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x42, 0x50, 0x46, 0x56, 0x31, 0x2E, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
