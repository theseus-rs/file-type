use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858483: FileFormat = FileFormat {
    id: 105_858_483,
    source_type: SourceType::Wikidata,
    name: "Sunplus Burn firmware update",
    extensions: &["brn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x55, 0x4E, 0x50, 0x20, 0x42, 0x55, 0x52, 0x4E, 0x20, 0x46, 0x49, 0x4C,
                    0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
