use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856008: FileFormat = FileFormat {
    id: 105_856_008,
    source_type: SourceType::Wikidata,
    name: "Data Model eXchange encoding format",
    extensions: &["dmx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x2D, 0x2D, 0x20, 0x64, 0x6D, 0x78, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                    0x64, 0x69, 0x6E, 0x67, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
