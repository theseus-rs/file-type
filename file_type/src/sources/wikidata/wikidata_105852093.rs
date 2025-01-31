use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852093: FileFormat = FileFormat {
    id: 105_852_093,
    puid: "wikidata/105852093",
    name: "Sonic Arranger module (v1.0)",
    extensions: &["sa"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4F, 0x41, 0x52, 0x56, 0x31, 0x2E, 0x30, 0x53, 0x54, 0x42, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
