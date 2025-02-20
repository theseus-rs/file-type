use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1086: FileType = FileType {
    file_format: &FileFormat {
        id: 1_086,
        source_type: SourceType::Pronom,
        name: "Macintosh PICT Image",
        extensions: &["pct", "pict", "pic"],
        media_types: &["image/x-pict"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(522),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x11, 0x02, 0xFF, 0x0C, 0x00])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 122,
        }],
    },
};
