use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854023: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_023,
        source_type: SourceType::Wikidata,
        name: "BeckerTools compressed archive/backup",
        extensions: &["btc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x65, 0x63, 0x6B, 0x65, 0x72, 0x54, 0x6F, 0x6F, 0x6C, 0x73, 0x20,
                        0x42, 0x54, 0x43, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x20, 0x20, 0x20,
                        0x28, 0x44, 0x61, 0x74, 0x61, 0x20, 0x42, 0x65, 0x63, 0x6B, 0x65, 0x72,
                        0x29, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
