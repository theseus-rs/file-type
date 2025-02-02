use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125364051: FileFormat = FileFormat {
    id: 125_364_051,
    source_type: SourceType::Wikidata,
    name: "Open Brush File",
    extensions: &["tilt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x69, 0x6C, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
