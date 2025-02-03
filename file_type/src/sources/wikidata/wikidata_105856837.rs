use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856837: FileFormat = FileFormat {
    id: 105_856_837,
    source_type: SourceType::Wikidata,
    name: "Logger Lite data",
    extensions: &["gmbl"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
