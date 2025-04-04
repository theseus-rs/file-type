use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851859: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_859,
        source_type: SourceType::Wikidata,
        name: "MagicQ Show",
        extensions: &["shw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x20, 0x4D, 0x61, 0x67, 0x69, 0x63, 0x51, 0x20, 0x53, 0x68, 0x6F,
                        0x77, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2E, 0x20, 0x43, 0x68, 0x61, 0x6D,
                        0x73, 0x79, 0x73, 0x20, 0x4C, 0x74, 0x64, 0x2E, 0x20, 0x20, 0x77, 0x77,
                        0x77, 0x2E, 0x63, 0x68, 0x61, 0x6D, 0x73, 0x79, 0x73, 0x2E, 0x63, 0x6F,
                        0x2E, 0x75, 0x6B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
