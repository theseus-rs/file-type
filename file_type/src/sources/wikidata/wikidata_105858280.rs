use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858280: FileFormat = FileFormat {
    id: 105_858_280,
    source_type: SourceType::Wikidata,
    name: "OpenSSL encrypted data",
    extensions: &["enc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x61, 0x6C, 0x74, 0x65, 0x64, 0x5F, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
