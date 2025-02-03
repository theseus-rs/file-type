use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861492: FileFormat = FileFormat {
    id: 105_861_492,
    source_type: SourceType::Wikidata,
    name: "Little Draw Drawing",
    extensions: &["ldw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x44, 0x52, 0x41, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
