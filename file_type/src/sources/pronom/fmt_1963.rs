use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1963: FileFormat = FileFormat {
    id: 2_828,
    puid: "fmt/1963",
    name: "NEC Thermo Tracer Image File",
    extensions: &["tmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x35, 0x31, 0x30, 0x30, 0x20, 0x33, 0x2E, 0x39, 0x4A, 0x20, 0x35, 0x31, 0x30,
                    0x32, 0x11, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
