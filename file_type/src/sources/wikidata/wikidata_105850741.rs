use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850741: FileFormat = FileFormat {
    id: 105_850_741,
    source_type: SourceType::Wikidata,
    name: "RAR registration data",
    extensions: &["key"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x41, 0x52, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74,
                    0x69, 0x6F, 0x6E, 0x20, 0x64, 0x61, 0x74, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
