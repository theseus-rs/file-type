use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856606: FileFormat = FileFormat {
    id: 105_856_606,
    source_type: SourceType::Wikidata,
    name: "WinFBE Project",
    extensions: &["wfbe"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x27, 0x00, 0x20, 0x00, 0x57, 0x00, 0x49, 0x00, 0x4E, 0x00, 0x46,
                    0x00, 0x42, 0x00, 0x45, 0x00, 0x20, 0x00, 0x50, 0x00, 0x52, 0x00, 0x4F, 0x00,
                    0x4A, 0x00, 0x45, 0x00, 0x43, 0x00, 0x54, 0x00, 0x20, 0x00, 0x46, 0x00, 0x49,
                    0x00, 0x4C, 0x00, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
