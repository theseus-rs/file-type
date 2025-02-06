use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853769: FileFormat = FileFormat {
    id: 105_853_769,
    source_type: SourceType::Wikidata,
    name: "PAQ8JD compressed archive",
    extensions: &["paq8jd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x61, 0x71, 0x38, 0x6A, 0x64, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
