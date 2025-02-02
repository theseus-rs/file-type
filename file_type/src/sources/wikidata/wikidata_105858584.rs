use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858584: FileFormat = FileFormat {
    id: 105_858_584,
    source_type: SourceType::Wikidata,
    name: "BitStore Metadata",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x69, 0x74, 0x53, 0x74, 0x6F, 0x72, 0x65, 0x2E, 0x4D, 0x65, 0x74, 0x61,
                    0x64, 0x61, 0x74, 0x61, 0x5F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
