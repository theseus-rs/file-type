use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855878: FileFormat = FileFormat {
    id: 105_855_878,
    source_type: SourceType::Wikidata,
    name: "360 Total Security data",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x30, 0x36, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
