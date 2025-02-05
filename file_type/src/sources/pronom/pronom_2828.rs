use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2828: FileFormat = FileFormat {
    id: 2_828,
    source_type: SourceType::Pronom,
    name: "NEC Thermo Tracer Image File",
    extensions: &["tmp"],
    media_types: &[],
    signatures: &[Signature {
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
