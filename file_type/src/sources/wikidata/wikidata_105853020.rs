use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853020: FileFormat = FileFormat {
    id: 105_853_020,
    source_type: SourceType::Wikidata,
    name: "SEM Snapshot",
    extensions: &["sem"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x05, 0x53, 0x50, 0x45, 0x43, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
