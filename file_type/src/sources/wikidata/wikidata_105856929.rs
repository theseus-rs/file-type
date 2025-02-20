use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856929: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_929,
        source_type: SourceType::Wikidata,
        name: "TADS 2 Game",
        extensions: &["gam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x41, 0x44, 0x53, 0x32, 0x20, 0x62, 0x69, 0x6E, 0x0A, 0x0D, 0x1A,
                        0x00, 0x76, 0x32, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
