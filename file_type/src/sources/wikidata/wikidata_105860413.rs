use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860413: FileFormat = FileFormat {
    id: 105_860_413,
    puid: "wikidata/105860413",
    name: "Windows compiled resource",
    extensions: &["res"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
                    0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
