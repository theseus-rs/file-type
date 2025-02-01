use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_67: FileFormat = FileFormat {
    id: 108,
    puid: "x-fmt/67",
    name: "OS/2 Presentation Manager Metafile (MET)",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(2),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD3, 0xA8, 0xA8])],
            },
        }],
    }],
    related_formats: &[],
};
