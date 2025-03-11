use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858621: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_621,
        source_type: SourceType::Wikidata,
        name: "Interleaf image format bitmap",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x89, 0x4F, 0x50, 0x53, 0x00, 0x04, 0x00, 0x4B, 0x00, 0x4B, 0x01, 0x02,
                        0x03, 0x04,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
