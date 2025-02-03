use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862934: FileFormat = FileFormat {
    id: 105_862_934,
    source_type: SourceType::Wikidata,
    name: "FAC Soundtracker module",
    extensions: &["mus"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x00, 0x80, 0xFF, 0xBF, 0x00, 0x80])],
            },
        }],
    }],
    related_formats: &[],
};
