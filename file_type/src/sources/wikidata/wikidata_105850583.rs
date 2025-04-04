use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850583: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_583,
        source_type: SourceType::Wikidata,
        name: "MFC ClassWizard info",
        extensions: &["clw"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x20, 0x43, 0x4C, 0x57, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x63,
                        0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x73, 0x20, 0x69, 0x6E, 0x66, 0x6F,
                        0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x6F, 0x72, 0x20,
                        0x74, 0x68, 0x65, 0x20, 0x4D, 0x46, 0x43, 0x20, 0x43, 0x6C, 0x61, 0x73,
                        0x73, 0x57, 0x69, 0x7A, 0x61, 0x72, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
