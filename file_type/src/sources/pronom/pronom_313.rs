use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_313: FileFormat = FileFormat {
    id: 313,
    source_type: SourceType::Pronom,
    name: "MapBrowser/MapWriter Vector Map Data",
    extensions: &["cbd"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x77, 0x00, 0x02, 0x40])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x77, 0x00, 0x33])],
                },
            }],
        },
    ],
    related_formats: &[],
};
