use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_634: FileFormat = FileFormat {
    id: 1_433,
    puid: "fmt/634",
    name: "Microsoft Compiled HTML Help",
    extensions: &["chm", "chw"],
    media_types: &["application/vnd.ms-htmlhelp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x54, 0x53, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
