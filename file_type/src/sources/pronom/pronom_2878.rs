use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2878: FileFormat = FileFormat {
    id: 2_878,
    source_type: SourceType::Pronom,
    name: "Husqvarna / Premier+ Embroidery Stitch File",
    extensions: &["vp4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x56, 0x70, 0x34, 0x25, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
