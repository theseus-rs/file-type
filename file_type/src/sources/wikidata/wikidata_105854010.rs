use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854010: FileFormat = FileFormat {
    id: 105_854_010,
    puid: "wikidata/105854010",
    name: "Midtown Madness 2 game data archive",
    extensions: &["ar"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x41, 0x56, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
