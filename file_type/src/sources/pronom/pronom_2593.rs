use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2593: FileType = FileType {
    file_format: &FileFormat {
        id: 2_593,
        source_type: SourceType::Pronom,
        name: "Microsoft PowerPoint Presentation",
        extensions: &["ppt"],
        media_types: &["application/vnd.ms-PowerPoint"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xED, 0xDE, 0xAD, 0x0B, 0x02, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_594,
        }],
    },
};
