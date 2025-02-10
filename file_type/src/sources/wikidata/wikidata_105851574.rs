use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851574: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_574,
        source_type: SourceType::Wikidata,
        name: "Klasik Translation Table",
        extensions: &["ttb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x2E, 0x54, 0x2E, 0x20, 0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20,
                        0x2D, 0x20, 0x54, 0x72, 0x61, 0x6E, 0x73, 0x6C, 0x61, 0x74, 0x69, 0x6F,
                        0x6E, 0x20, 0x54, 0x61, 0x62, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
