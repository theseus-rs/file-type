use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2740: FileType = FileType {
    file_format: &FileFormat {
        id: 2_740,
        source_type: SourceType::Pronom,
        name: "Encapsulated PostScript File Format",
        extensions: &["eps", "epsf"],
        media_types: &["application/postscript"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                        0x2E, 0x30, 0x20, 0x45, 0x50, 0x53, 0x46, 0x2D, 0x32, 0x2E, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 773,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 331,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 332,
            },
        ],
    },
};
