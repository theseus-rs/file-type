use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2376: FileFormat = FileFormat {
    id: 2_376,
    source_type: SourceType::Pronom,
    name: "NetWare Loadable Module",
    extensions: &["nlm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x65, 0x74, 0x57, 0x61, 0x72, 0x65, 0x20, 0x4C, 0x6F, 0x61, 0x64, 0x61,
                    0x62, 0x6C, 0x65, 0x20, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x1A, 0x04,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
