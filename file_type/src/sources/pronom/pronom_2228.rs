use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2228: FileFormat = FileFormat {
    id: 2_228,
    source_type: SourceType::Pronom,
    name: "Flow Charting",
    extensions: &["fcx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x43, 0x36, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
