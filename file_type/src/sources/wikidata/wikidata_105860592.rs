use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860592: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_592,
        source_type: SourceType::Wikidata,
        name: "WinAsks Player Report (v2.00)",
        extensions: &["rpt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1A, 0x00, 0x57, 0x69, 0x6E, 0x41, 0x73, 0x6B, 0x73, 0x20, 0x50, 0x6C,
                        0x61, 0x79, 0x65, 0x72, 0x20, 0x52, 0x65, 0x70, 0x6F, 0x72, 0x74, 0x20,
                        0x46, 0x69, 0x6C, 0x65, 0x04, 0x00, 0x32, 0x2E, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
