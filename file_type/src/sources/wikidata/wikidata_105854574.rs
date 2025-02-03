use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854574: FileFormat = FileFormat {
    id: 105_854_574,
    source_type: SourceType::Wikidata,
    name: "Aodix project",
    extensions: &["adx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x34, 0x58, 0x44, 0x41, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
