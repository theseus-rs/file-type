use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856623: FileFormat = FileFormat {
    id: 105_856_623,
    puid: "wikidata/105856623",
    name: "Earache: Extreme Metal Racing game data",
    extensions: &["wad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x41, 0x44, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
