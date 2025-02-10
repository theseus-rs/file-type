use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866843: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_843,
        source_type: SourceType::Wikidata,
        name: "X-Plane Draped Polygon",
        extensions: &["pol"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x38, 0x35, 0x30, 0x0A, 0x44, 0x52, 0x41, 0x50, 0x45, 0x44, 0x5F,
                        0x50, 0x4F, 0x4C, 0x59, 0x47, 0x4F, 0x4E, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
