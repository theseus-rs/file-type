use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866499: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_499,
        source_type: SourceType::Wikidata,
        name: "Messenger Plus! Backup Configuration",
        extensions: &["pld"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4C, 0x55, 0x53, 0x5F, 0x50, 0x52, 0x45, 0x46, 0x5F, 0x50, 0x41,
                        0x43, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
