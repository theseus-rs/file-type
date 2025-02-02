use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850561: FileFormat = FileFormat {
    id: 105_850_561,
    source_type: SourceType::Wikidata,
    name: "Classical Text Editor document (v7)",
    extensions: &["cte"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x01, 0xFF, 0x01, 0x43, 0x54, 0x45, 0x5F, 0x37, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
