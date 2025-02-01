use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1957: FileFormat = FileFormat {
    id: 2_822,
    puid: "fmt/1957",
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
