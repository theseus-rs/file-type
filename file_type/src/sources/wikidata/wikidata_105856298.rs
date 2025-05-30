use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856298: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_298,
        source_type: SourceType::Wikidata,
        name: "Runtime Software compressed disk image",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1A, 0x52, 0x54, 0x53, 0x20, 0x43, 0x4F, 0x4D, 0x50, 0x52, 0x45, 0x53,
                        0x53, 0x45, 0x44, 0x20, 0x49, 0x4D, 0x41, 0x47, 0x45, 0x20, 0x56, 0x31,
                        0x2E, 0x30, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
