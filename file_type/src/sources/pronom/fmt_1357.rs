use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1357: FileFormat = FileFormat {
    id: 2_175,
    puid: "fmt/1357",
    name: "Virtual Format (Vector)",
    extensions: &["vrt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4F, 0x47, 0x52, 0x56, 0x52, 0x54, 0x44, 0x61, 0x74, 0x61, 0x53, 0x6F,
                    0x75, 0x72, 0x63, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
