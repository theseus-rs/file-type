use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854408: FileFormat = FileFormat {
    id: 105_854_408,
    source_type: SourceType::Wikidata,
    name: "LU library",
    extensions: &["lbr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
