use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1873: FileType = FileType {
    file_format: &FileFormat {
        id: 1_873,
        source_type: SourceType::Pronom,
        name: "Portable Database",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x3C, 0x3C, 0x50, 0x44, 0x42, 0x3A, 0x33, 0x3E, 0x3E, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_872,
        }],
    },
};
