use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857459: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_459,
        source_type: SourceType::Wikidata,
        name: "3-D Professional script",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x53, 0x43, 0x52, 0x49, 0x50, 0x54, 0x31, 0x0A, 0x0A, 0x53,
                        0x45, 0x54, 0x44, 0x49, 0x52, 0x45, 0x43, 0x54, 0x4F, 0x52, 0x59, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
