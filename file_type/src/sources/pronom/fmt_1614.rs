use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1614: FileFormat = FileFormat {
    id: 2_441,
    puid: "fmt/1614",
    name: "Esri ArcExplorer Project File",
    extensions: &["aep"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
