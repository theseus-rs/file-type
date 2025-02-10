use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1766: FileType = FileType {
    file_format: &FileFormat {
        id: 1_766,
        source_type: SourceType::Pronom,
        name: "Mobile eXtensible Music Format",
        extensions: &["mxmf"],
        media_types: &["audio/mobile-xmf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x4D, 0x46, 0x5F, 0x32, 0x2E, 0x30, 0x30, 0x00, 0x00, 0x00, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_513,
        }],
    },
};
