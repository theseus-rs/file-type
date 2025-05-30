use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852113: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_113,
        source_type: SourceType::Wikidata,
        name: "Structured Titles subtitles (UTF-8)",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65,
                        0x64, 0x20, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
