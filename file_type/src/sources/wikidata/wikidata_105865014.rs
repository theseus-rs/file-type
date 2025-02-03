use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865014: FileFormat = FileFormat {
    id: 105_865_014,
    source_type: SourceType::Wikidata,
    name: "TomTom PNA map info",
    extensions: &["pna"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x44, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
