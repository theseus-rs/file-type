use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2420: FileFormat = FileFormat {
    id: 2_420,
    source_type: SourceType::Pronom,
    name: "ASEG-GDF2- Data Definition File",
    extensions: &["dfn"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x45, 0x46, 0x4E])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x0D, 0x0A,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
