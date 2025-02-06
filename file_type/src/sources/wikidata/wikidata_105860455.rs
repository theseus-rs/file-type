use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860455: FileFormat = FileFormat {
    id: 105_860_455,
    source_type: SourceType::Wikidata,
    name: "Rage Maker workspace",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x72, 0x61, 0x67, 0x65, 0x6D, 0x61, 0x6B, 0x65, 0x72, 0x3E, 0x0A, 0x20,
                    0x20, 0x3C, 0x70, 0x61, 0x6E, 0x65, 0x6C, 0x73, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
