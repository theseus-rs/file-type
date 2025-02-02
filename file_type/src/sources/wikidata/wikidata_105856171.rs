use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856171: FileFormat = FileFormat {
    id: 105_856_171,
    source_type: SourceType::Wikidata,
    name: "Propilkki data",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x72, 0x6F, 0x20, 0x50, 0x69, 0x6C, 0x6B, 0x6B, 0x69, 0x20, 0x64, 0x61,
                    0x74, 0x61, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
