use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861960: FileFormat = FileFormat {
    id: 105_861_960,
    puid: "wikidata/105861960",
    name: "Quake 3 Model (newer)",
    extensions: &["md4"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x44, 0x50, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
