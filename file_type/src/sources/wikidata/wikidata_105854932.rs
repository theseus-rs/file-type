use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854932: FileFormat = FileFormat {
    id: 105_854_932,
    source_type: SourceType::Wikidata,
    name: "BRL-CAD Geometry (ASCII)",
    extensions: &["asc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x69, 0x74, 0x6C, 0x65, 0x20, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
