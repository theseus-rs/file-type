use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2444: FileFormat = FileFormat {
    id: 2_444,
    source_type: SourceType::Pronom,
    name: "Devicetree Blob (DTB)",
    extensions: &["dtb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD0, 0x0D, 0xFE, 0xED])],
            },
        }],
    }],
    related_formats: &[],
};
