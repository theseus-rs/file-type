use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_350: FileFormat = FileFormat {
    id: 514,
    puid: "x-fmt/350",
    name: "OmniPage Pro Document",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x43, 0x4D, 0x45, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
