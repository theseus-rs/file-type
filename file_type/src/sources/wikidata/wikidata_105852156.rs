use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852156: FileFormat = FileFormat {
    id: 105_852_156,
    source_type: SourceType::Wikidata,
    name: "DemoManiac Script",
    extensions: &["script"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4D, 0x56, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
