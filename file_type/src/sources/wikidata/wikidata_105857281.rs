use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857281: FileFormat = FileFormat {
    id: 105_857_281,
    source_type: SourceType::Wikidata,
    name: "Polar Heart Rate Monitor format",
    extensions: &["hrm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x61, 0x72, 0x61, 0x6D, 0x73, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
