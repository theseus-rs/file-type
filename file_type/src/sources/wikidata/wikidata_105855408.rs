use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855408: FileFormat = FileFormat {
    id: 105_855_408,
    puid: "wikidata/105855408",
    name: "Face The Music module",
    extensions: &["ftm"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x54, 0x4D, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
