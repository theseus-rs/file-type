use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852213: FileFormat = FileFormat {
    id: 105_852_213,
    source_type: SourceType::Wikidata,
    name: "Sage Binary",
    extensions: &["sab"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x07, 0x1F, 0x0D, 0x09, 0x00, 0x0D, 0x02, 0x00, 0x20,
                    0x0B, 0x13, 0x43, 0x4F, 0x4D, 0x50, 0x49, 0x4C, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                    0x2D, 0x55, 0x53, 0x45, 0x52, 0x2D, 0x49, 0x44, 0x0B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
