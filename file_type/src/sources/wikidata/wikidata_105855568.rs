use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855568: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_568,
        source_type: SourceType::Wikidata,
        name: "Norton pcAnywhere Online Services list",
        extensions: &["os6"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x6F, 0x72, 0x74, 0x6F, 0x6E, 0x20, 0x70, 0x63, 0x41, 0x4E, 0x59,
                        0x57, 0x48, 0x45, 0x52, 0x45, 0x20, 0x4F, 0x6E, 0x6C, 0x69, 0x6E, 0x65,
                        0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x20, 0x4C, 0x69,
                        0x73, 0x74, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
