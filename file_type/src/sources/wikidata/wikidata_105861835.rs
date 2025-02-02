use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861835: FileFormat = FileFormat {
    id: 105_861_835,
    source_type: SourceType::Wikidata,
    name: "Unique Development Samples",
    extensions: &["smp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x4C, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
