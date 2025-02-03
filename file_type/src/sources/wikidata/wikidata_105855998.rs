use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855998: FileFormat = FileFormat {
    id: 105_855_998,
    source_type: SourceType::Wikidata,
    name: "Dis bytecode (not signed)",
    extensions: &["dis"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x0C, 0x80, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
