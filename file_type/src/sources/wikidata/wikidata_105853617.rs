use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853617: FileFormat = FileFormat {
    id: 105_853_617,
    puid: "wikidata/105853617",
    name: "ASCII Test Data Format",
    extensions: &["atdf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
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
