use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851677: FileFormat = FileFormat {
    id: 105_851_677,
    source_type: SourceType::Wikidata,
    name: "StepMania step file",
    extensions: &["sm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
