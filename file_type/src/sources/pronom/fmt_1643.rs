use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1643: FileFormat = FileFormat {
    id: 2_470,
    puid: "fmt/1643",
    name: "Lenel Network Video Recorder File",
    extensions: &["lnr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4E, 0x52, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
