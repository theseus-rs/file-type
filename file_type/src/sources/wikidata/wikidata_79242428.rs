use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_79242428: FileFormat = FileFormat {
    id: 79_242_428,
    source_type: SourceType::Wikidata,
    name: "Antenna Data File",
    extensions: &["adf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x56, 0x4E, 0x55, 0x4D, 0x3A, 0x2C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
