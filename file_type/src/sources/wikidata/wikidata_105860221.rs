use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860221: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_221,
        source_type: SourceType::Wikidata,
        name: "RegCleaner v4.3 Language File",
        extensions: &["rlg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x52, 0x65, 0x67, 0x43, 0x6C, 0x65, 0x61, 0x6E, 0x65, 0x72, 0x20,
                        0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65, 0x50, 0x61, 0x63, 0x6B,
                        0x2A, 0x0D, 0x0A, 0x2A, 0x52, 0x47, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x20, 0x3A, 0x20, 0x34, 0x2E, 0x33, 0x2A, 0x0D, 0x0A, 0x0D,
                        0x0A, 0x2F, 0x2F, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
