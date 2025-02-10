use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857067: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_067,
        source_type: SourceType::Wikidata,
        name: "Greenfish Icon Editor Pro",
        extensions: &["gfi", "gfie"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x67, 0x66, 0x64, 0x74, 0x3C, 0x01, 0x5C, 0x00, 0x00, 0x00, 0x00, 0x3C,
                        0x08, 0x6D, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
