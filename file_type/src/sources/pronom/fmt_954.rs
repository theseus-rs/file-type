use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_954: FileFormat = FileFormat {
    id: 1_759,
    puid: "fmt/954",
    name: "Adaptive Multi-Rate Wideband Audio",
    extensions: &["awb"],
    media_types: &["audio/amr-wb"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x21, 0x41, 0x4D, 0x52, 0x2D, 0x57, 0x42, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
