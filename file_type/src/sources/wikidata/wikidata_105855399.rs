use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855399: FileFormat = FileFormat {
    id: 105_855_399,
    source_type: SourceType::Wikidata,
    name: "DemoManiac Font",
    extensions: &["font"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x6F, 0x4E, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
