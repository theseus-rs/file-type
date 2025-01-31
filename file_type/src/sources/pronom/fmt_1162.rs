use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1162: FileFormat = FileFormat {
    id: 1_972,
    puid: "fmt/1162",
    name: "Folio Flat File",
    extensions: &["fff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x43, 0x4D, 0x3E])],
                },
            },
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x6F, 0x6C, 0x69, 0x6F, 0x2C, 0x46, 0x46, 0x46,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
