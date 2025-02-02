use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2822: FileFormat = FileFormat {
    id: 2_822,
    source_type: SourceType::Pronom,
    name: "Program Embroidery Stitch (PES) File",
    extensions: &["pes"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x50, 0x45, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
