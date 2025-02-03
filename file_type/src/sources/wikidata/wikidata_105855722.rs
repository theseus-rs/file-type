use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855722: FileFormat = FileFormat {
    id: 105_855_722,
    source_type: SourceType::Wikidata,
    name: "Flashback Object",
    extensions: &["obj"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE6, 0x00, 0x98, 0x03, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
