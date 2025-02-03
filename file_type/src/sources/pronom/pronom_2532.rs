use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2532: FileFormat = FileFormat {
    id: 2_532,
    source_type: SourceType::Pronom,
    name: "ESRI Attribute Index Files",
    extensions: &["ain"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0xCE, 0xAE, 0x19, 0x40, 0x06, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
