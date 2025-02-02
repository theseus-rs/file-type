use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853715: FileFormat = FileFormat {
    id: 105_853_715,
    source_type: SourceType::Wikidata,
    name: "Liquid Entertainment H2O game data archive",
    extensions: &["h2o"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x49, 0x51, 0x44, 0x4C, 0x48, 0x32, 0x4F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
