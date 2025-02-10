use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859879: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_879,
        source_type: SourceType::Wikidata,
        name: "Nokia Saved SMS (Unicode)",
        extensions: &["vmg"],
        media_types: &["text/x-vMessage"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x00, 0x45, 0x00, 0x47, 0x00, 0x49, 0x00, 0x4E, 0x00, 0x3A, 0x00,
                        0x56, 0x00, 0x4D, 0x00, 0x53, 0x00, 0x47,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
