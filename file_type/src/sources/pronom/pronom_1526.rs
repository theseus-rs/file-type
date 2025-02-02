use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1526: FileFormat = FileFormat {
    id: 1_526,
    source_type: SourceType::Pronom,
    name: "Cartesian Perceptual Compression image format",
    extensions: &["cpi", "cpc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x50, 0x43, 0xB2])],
            },
        }],
    }],
    related_formats: &[],
};
