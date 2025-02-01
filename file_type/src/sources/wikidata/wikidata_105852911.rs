use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852911: FileFormat = FileFormat {
    id: 105_852_911,
    puid: "wikidata/105852911",
    name: "Postal game data Archive",
    extensions: &["sak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x41, 0x4B, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
