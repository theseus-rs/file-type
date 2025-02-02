use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856653: FileFormat = FileFormat {
    id: 105_856_653,
    source_type: SourceType::Wikidata,
    name: "Wirecast Setup",
    extensions: &["wcst"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x76, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
