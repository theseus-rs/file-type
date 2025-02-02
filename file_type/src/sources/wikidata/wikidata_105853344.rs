use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853344: FileFormat = FileFormat {
    id: 105_853_344,
    source_type: SourceType::Wikidata,
    name: "StepMania Song (with rem)",
    extensions: &["ssc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2F])],
            },
        }],
    }],
    related_formats: &[],
};
