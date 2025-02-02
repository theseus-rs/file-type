use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855096: FileFormat = FileFormat {
    id: 105_855_096,
    source_type: SourceType::Wikidata,
    name: "Morpheus layout - project",
    extensions: &["aml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
