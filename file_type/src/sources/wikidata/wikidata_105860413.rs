use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860413: FileFormat = FileFormat {
    id: 105_860_413,
    source_type: SourceType::Wikidata,
    name: "Windows compiled resource",
    extensions: &["res"],
    media_types: &[],
    signatures: &[Signature {
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
