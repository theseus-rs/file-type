use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865040: FileFormat = FileFormat {
    id: 105_865_040,
    source_type: SourceType::Wikidata,
    name: "Yamaha RS7000 OS image",
    extensions: &["pgm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x4D, 0x53, 0x78, 0x20, 0x4F, 0x53, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x00,
                    0x00, 0x00, 0x00, 0x30, 0x30, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
