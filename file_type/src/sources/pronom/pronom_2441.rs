use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2441: FileFormat = FileFormat {
    id: 2_441,
    source_type: SourceType::Pronom,
    name: "Esri ArcExplorer Project File",
    extensions: &["aep"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x41, 0x52, 0x43, 0x45, 0x58, 0x50, 0x4C, 0x4F, 0x52, 0x45, 0x52, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
