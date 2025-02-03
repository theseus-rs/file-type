use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862143: FileFormat = FileFormat {
    id: 105_862_143,
    source_type: SourceType::Wikidata,
    name: "Media Center Markup Language",
    extensions: &["mcml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x63, 0x6D, 0x6C, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
