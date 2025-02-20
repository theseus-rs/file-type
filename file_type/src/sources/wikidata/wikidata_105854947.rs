use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854947: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_947,
        source_type: SourceType::Wikidata,
        name: "ABC FlowCharter Workspace",
        extensions: &["afw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x18, 0x00, 0x4A, 0x46, 0x4F, 0x00, 0x00, 0x1B, 0xDE, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
