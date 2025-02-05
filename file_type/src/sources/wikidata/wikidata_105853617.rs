use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853617: FileFormat = FileFormat {
    id: 105_853_617,
    source_type: SourceType::Wikidata,
    name: "ASCII Test Data Format",
    extensions: &["atdf"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x41, 0x52, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
