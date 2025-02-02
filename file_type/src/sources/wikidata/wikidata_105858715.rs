use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858715: FileFormat = FileFormat {
    id: 105_858_715,
    source_type: SourceType::Wikidata,
    name: "BootSkin Vista theme",
    extensions: &["bootskin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x53, 0x56, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
