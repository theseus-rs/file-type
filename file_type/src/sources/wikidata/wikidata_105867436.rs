use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867436: FileFormat = FileFormat {
    id: 105_867_436,
    source_type: SourceType::Wikidata,
    name: "Drishti Processed Volume (header)",
    extensions: &["nc"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x44, 0x72, 0x69,
                    0x73, 0x68, 0x74, 0x69, 0x5F, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
