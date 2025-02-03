use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858128: FileFormat = FileFormat {
    id: 105_858_128,
    source_type: SourceType::Wikidata,
    name: "Card Image Backup format",
    extensions: &["cib"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x43, 0x49, 0x42, 0x3E, 0xC1, 0xB0])],
            },
        }],
    }],
    related_formats: &[],
};
