use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1433: FileFormat = FileFormat {
    id: 1_433,
    source_type: SourceType::Pronom,
    name: "Microsoft Compiled HTML Help",
    extensions: &["chm", "chw"],
    media_types: &["application/vnd.ms-htmlhelp"],
    signatures: &[Signature {
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
