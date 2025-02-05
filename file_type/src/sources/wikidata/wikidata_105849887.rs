use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849887: FileFormat = FileFormat {
    id: 105_849_887,
    source_type: SourceType::Wikidata,
    name: "NTI CD Maker layout file",
    extensions: &["cdm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0E, 0x4E, 0x54, 0x49, 0x5F, 0x43, 0x44, 0x4C, 0x41, 0x59, 0x4F, 0x55, 0x54,
                    0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
