use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858429: FileFormat = FileFormat {
    id: 105_858_429,
    puid: "wikidata/105858429",
    name: "Windows Event Viewer Log",
    extensions: &["evt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x30, 0x00, 0x00, 0x00, 0x4C, 0x66, 0x4C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
