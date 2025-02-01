use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858239: FileFormat = FileFormat {
    id: 105_858_239,
    puid: "wikidata/105858239",
    name: "Fashion Tracker module",
    extensions: &["ex"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x13, 0xFC, 0x00, 0x40])],
            },
        }],
    }],
    related_formats: &[],
};
