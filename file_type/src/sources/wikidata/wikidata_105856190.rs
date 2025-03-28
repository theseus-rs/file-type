use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856190: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_190,
        source_type: SourceType::Wikidata,
        name: "Vista Digital Elevation Map",
        extensions: &["dem"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x69, 0x73, 0x74, 0x61, 0x20, 0x44, 0x45, 0x4D, 0x20, 0x46, 0x69,
                        0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
