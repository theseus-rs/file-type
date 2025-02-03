use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72177532: FileFormat = FileFormat {
    id: 72_177_532,
    source_type: SourceType::Wikidata,
    name: "kRAW Audio Stream",
    extensions: &["kraw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6B, 0x52, 0x41, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
