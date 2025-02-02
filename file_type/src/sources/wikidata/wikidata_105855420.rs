use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855420: FileFormat = FileFormat {
    id: 105_855_420,
    source_type: SourceType::Wikidata,
    name: "FCE-Ultra movie capture",
    extensions: &["fcm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x43, 0x4D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
