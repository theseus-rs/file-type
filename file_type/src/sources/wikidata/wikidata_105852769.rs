use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852769: FileFormat = FileFormat {
    id: 105_852_769,
    source_type: SourceType::Wikidata,
    name: "STAT compressed",
    extensions: &["st"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x46, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
