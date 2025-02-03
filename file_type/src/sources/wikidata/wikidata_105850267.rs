use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850267: FileFormat = FileFormat {
    id: 105_850_267,
    source_type: SourceType::Wikidata,
    name: "StepMania Course",
    extensions: &["crs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x43, 0x4F, 0x55, 0x52, 0x53, 0x45, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
