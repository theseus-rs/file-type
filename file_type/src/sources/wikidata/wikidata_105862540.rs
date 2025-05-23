use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862540: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_540,
        source_type: SourceType::Wikidata,
        name: "Montage Page",
        extensions: &["pg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A,
                        0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x20, 0x50, 0x41, 0x47, 0x45, 0x20, 0x41,
                        0x54, 0x54, 0x52, 0x49, 0x42, 0x55, 0x54, 0x45, 0x53, 0x20, 0x2A, 0x2A,
                        0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A,
                        0x2A, 0x2A, 0x2A, 0x0A, 0x2F, 0x4B, 0x45, 0x59, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
