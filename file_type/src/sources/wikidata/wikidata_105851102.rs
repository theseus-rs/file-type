use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851102: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_102,
        source_type: SourceType::Wikidata,
        name: "Plan-80 spreadsheet (with options)",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3A, 0x4F, 0x50, 0x54, 0x49, 0x4F, 0x4E, 0x53, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
