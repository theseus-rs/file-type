use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_16965621: FileType = FileType {
    file_format: &FileFormat {
        id: 16_965_621,
        source_type: SourceType::Wikidata,
        name: "Video Image Communication And Retrieval",
        extensions: &["img", "vic", "vicar"],
        media_types: &["image/vicar"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x42, 0x4C, 0x53, 0x49, 0x5A, 0x45, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
