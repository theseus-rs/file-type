use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857414: FileFormat = FileFormat {
    id: 105_857_414,
    source_type: SourceType::Wikidata,
    name: "Jolly Print Studio graphic",
    extensions: &["jps"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x4A, 0x50, 0x53, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
