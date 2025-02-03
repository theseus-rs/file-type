use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849651: FileFormat = FileFormat {
    id: 105_849_651,
    source_type: SourceType::Wikidata,
    name: "Comic Collector Collection data",
    extensions: &["cmc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x06, 0x0E, 0x43, 0x6F, 0x6C, 0x6C, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x7A, 0x2E,
                    0x63, 0x6F, 0x6D, 0x06, 0x05, 0x43, 0x6F, 0x6D, 0x69, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
