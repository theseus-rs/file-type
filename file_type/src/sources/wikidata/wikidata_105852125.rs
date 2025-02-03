use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852125: FileFormat = FileFormat {
    id: 105_852_125,
    source_type: SourceType::Wikidata,
    name: "Snzip compressed (snzip format)",
    extensions: &["snz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4E, 0x5A, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
