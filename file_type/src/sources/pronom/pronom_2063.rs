use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2063: FileType = FileType {
    file_format: &FileFormat {
        id: 2_063,
        source_type: SourceType::Pronom,
        name: "ZFO (Proof of Delivery) File",
        extensions: &["zfo"],
        media_types: &["application/vnd.software602.filler.form-xml-zip"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x69, 0x73, 0x64, 0x73, 0x2E,
                        0x63, 0x7A, 0x65, 0x63, 0x68, 0x70, 0x6F, 0x69, 0x6E, 0x74, 0x2E, 0x63,
                        0x7A, 0x2F, 0x76, 0x32, 0x30, 0x2F, 0x64, 0x65, 0x6C, 0x69, 0x76, 0x65,
                        0x72, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_469,
        }],
    },
};
