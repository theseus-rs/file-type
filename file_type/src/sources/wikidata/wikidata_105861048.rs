use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861048: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_048,
        source_type: SourceType::Wikidata,
        name: "Advanced Layouter project",
        extensions: &["lay"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x58, 0x46, 0x2D, 0x53, 0x4C, 0x30, 0x30, 0x4F, 0x70, 0x74, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
