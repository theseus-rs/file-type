use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856763: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_763,
        source_type: SourceType::Wikidata,
        name: "Ellisys Visual USB Data",
        extensions: &["ufo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x6C, 0x6C, 0x69, 0x73, 0x79, 0x73, 0x20, 0x56, 0x69, 0x73, 0x75,
                        0x61, 0x6C, 0x20, 0x55, 0x53, 0x42, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20,
                        0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
