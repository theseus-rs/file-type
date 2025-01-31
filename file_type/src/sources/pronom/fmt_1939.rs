use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1939: FileFormat = FileFormat {
    id: 2_800,
    puid: "fmt/1939",
    name: "Auto FX PhotoGraphic Edges Image File",
    extensions: &["afx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x88, 0x40, 0x47, 0x59, 0x0C, 0x0B, 0x1B, 0x0B, 0x00, 0x01, 0x01, 0x02, 0x01,
                    0x01, 0x05, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
