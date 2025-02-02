use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854711: FileFormat = FileFormat {
    id: 105_854_711,
    source_type: SourceType::Wikidata,
    name: "Squash compressed archive",
    extensions: &["arh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x01, 0x09, 0x4F, 0x63, 0x74, 0x53, 0x71, 0x75, 0x09, 0x01, 0x79, 0x76,
                    0x65, 0x72, 0x79, 0x00, 0x6D, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
