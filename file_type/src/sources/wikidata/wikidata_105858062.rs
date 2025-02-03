use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858062: FileFormat = FileFormat {
    id: 105_858_062,
    source_type: SourceType::Wikidata,
    name: "Virtual98 harddisk image",
    extensions: &["hdd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x48, 0x44, 0x31, 0x2E, 0x30, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
