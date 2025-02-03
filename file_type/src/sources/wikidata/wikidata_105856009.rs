use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856009: FileFormat = FileFormat {
    id: 105_856_009,
    source_type: SourceType::Wikidata,
    name: "DAZ Studio script",
    extensions: &["dsa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x20, 0x44, 0x41, 0x5A, 0x20, 0x53, 0x74, 0x75, 0x64, 0x69, 0x6F,
                    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
