use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_137039604: FileType = FileType {
    file_format: &FileFormat {
        id: 137_039_604,
        source_type: SourceType::Wikidata,
        name: "Tldraw file",
        extensions: &["tldr"],
        media_types: &["application/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x74, 0x6C, 0x64, 0x72, 0x61, 0x77, 0x46, 0x69, 0x6C, 0x65, 0x46, 0x6F,
                        0x72, 0x6D, 0x61, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
