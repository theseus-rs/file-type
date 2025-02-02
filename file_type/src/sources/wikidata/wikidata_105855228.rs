use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855228: FileFormat = FileFormat {
    id: 105_855_228,
    source_type: SourceType::Wikidata,
    name: "FastDir-like quick directory lookup data",
    extensions: &["fd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x43, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
