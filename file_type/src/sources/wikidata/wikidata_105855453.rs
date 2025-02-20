use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855453: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_453,
        source_type: SourceType::Wikidata,
        name: "LaTeX Font Definition",
        extensions: &["fd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x50, 0x72, 0x6F, 0x76, 0x69, 0x64, 0x65, 0x73, 0x46, 0x69, 0x6C,
                        0x65, 0x7B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
