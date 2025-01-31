use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1868: FileFormat = FileFormat {
    id: 2_722,
    puid: "fmt/1868",
    name: "Leapfrog Geo 3D Scene Format",
    extensions: &["lfsc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x4C, 0x65, 0x61, 0x70, 0x66, 0x72, 0x6F, 0x67, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
