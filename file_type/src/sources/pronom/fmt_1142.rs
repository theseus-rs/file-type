use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1142: FileFormat = FileFormat {
    id: 1_952,
    puid: "fmt/1142",
    name: "VectorWorks Plugin or Script",
    extensions: &["vso", "vst", "vsm"],
    media_types: &["application/vnd.vectorworks"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x43, 0x56, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
