use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858400: FileFormat = FileFormat {
    id: 105_858_400,
    source_type: SourceType::Wikidata,
    name: "QLAY MDV image",
    extensions: &["mdv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
