use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2453: FileFormat = FileFormat {
    id: 2_453,
    source_type: SourceType::Pronom,
    name: "MicroStation Symbology Resource File",
    extensions: &["rsc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x63, 0x72, 0x6F, 0x53, 0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                    0x52, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x20, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
