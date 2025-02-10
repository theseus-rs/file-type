use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851105: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_105,
        source_type: SourceType::Wikidata,
        name: "Cache Directory Tagging Standard",
        extensions: &["tag"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x69, 0x67, 0x6E, 0x61, 0x74, 0x75, 0x72, 0x65, 0x3A, 0x20, 0x38,
                        0x61, 0x34, 0x37, 0x37, 0x66, 0x35, 0x39, 0x37, 0x64, 0x32, 0x38, 0x64,
                        0x31, 0x37, 0x32, 0x37, 0x38, 0x39, 0x66, 0x30, 0x36, 0x38, 0x38, 0x36,
                        0x38, 0x30, 0x36, 0x62, 0x63, 0x35, 0x35,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
