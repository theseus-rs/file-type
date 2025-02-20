use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860362: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_362,
        source_type: SourceType::Wikidata,
        name: "TNTmips Project (Windows)",
        extensions: &["rvc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x56, 0x43, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69, 0x6C,
                        0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
