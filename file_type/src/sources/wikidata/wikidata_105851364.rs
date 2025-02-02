use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851364: FileFormat = FileFormat {
    id: 105_851_364,
    source_type: SourceType::Wikidata,
    name: "SDLTRS State Save",
    extensions: &["t8s"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x6C, 0x64, 0x74, 0x72, 0x73, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20,
                    0x53, 0x61, 0x76, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
