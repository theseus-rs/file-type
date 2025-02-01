use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852801: FileFormat = FileFormat {
    id: 105_852_801,
    puid: "wikidata/105852801",
    name: "Just Cause game data archive",
    extensions: &["sab"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x57, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
