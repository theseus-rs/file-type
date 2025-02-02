use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867015: FileFormat = FileFormat {
    id: 105_867_015,
    source_type: SourceType::Wikidata,
    name: "NeoDesk icon (compressed)",
    extensions: &["nic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x4E, 0x49, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
