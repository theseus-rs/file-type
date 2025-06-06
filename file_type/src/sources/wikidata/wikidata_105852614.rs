use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852614: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_614,
        source_type: SourceType::Wikidata,
        name: "CPBackup backup Settings (v8.x)",
        extensions: &["set"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x43, 0x42, 0x53, 0x49, 0x47, 0x5D, 0x38,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
