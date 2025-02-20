use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863064: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_064,
        source_type: SourceType::Wikidata,
        name: "Meal-Master Format recipe",
        extensions: &["mmf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x4D, 0x4D, 0x4D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x20, 0x52,
                        0x65, 0x63, 0x69, 0x70, 0x65, 0x20, 0x76, 0x69, 0x61, 0x20, 0x4D, 0x65,
                        0x61, 0x6C, 0x2D, 0x4D, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x28, 0x74,
                        0x6D, 0x29, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
