use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851707: FileFormat = FileFormat {
    id: 105_851_707,
    source_type: SourceType::Wikidata,
    name: "DVD Subtitles",
    extensions: &["sub"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x01, 0xBA, 0x44, 0x02, 0xC4, 0x82, 0x04, 0xA9, 0x01, 0x89, 0xC3,
                    0xF8, 0x00, 0x00, 0x01, 0xBD, 0x07, 0xEC, 0x81, 0x80, 0x05, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
