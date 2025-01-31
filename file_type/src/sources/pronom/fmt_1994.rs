use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1994: FileFormat = FileFormat {
    id: 2_868,
    puid: "fmt/1994",
    name: "Sibelius Scorch",
    extensions: &["sco"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0F, 0x43, 0x43, 0x53, 0x43, 0x4F, 0x52, 0x43, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
