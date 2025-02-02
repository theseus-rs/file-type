use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854922: FileFormat = FileFormat {
    id: 105_854_922,
    source_type: SourceType::Wikidata,
    name: "DVSM Sample audio",
    extensions: &["dvs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x56, 0x53, 0x4D, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
