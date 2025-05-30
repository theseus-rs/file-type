use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858446: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_446,
        source_type: SourceType::Wikidata,
        name: "Keyhole - Google Earth Placemark",
        extensions: &["eta"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6E,
                        0x20, 0x4B, 0x65, 0x79, 0x68, 0x6F, 0x6C, 0x65, 0x20, 0x45, 0x54, 0x41,
                        0x20, 0x66, 0x69, 0x6C, 0x65, 0x2E, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63,
                        0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20,
                        0x61, 0x75, 0x74, 0x6F, 0x6D, 0x61, 0x74, 0x69, 0x63, 0x61, 0x6C, 0x6C,
                        0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
