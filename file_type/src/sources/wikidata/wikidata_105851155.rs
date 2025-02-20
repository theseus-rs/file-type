use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851155: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_155,
        source_type: SourceType::Wikidata,
        name: "ArtCAM Toolpath Template (old)",
        extensions: &["tpl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x18, 0x41, 0x72, 0x74, 0x43, 0x41, 0x4D, 0x20, 0x54, 0x6F, 0x6F, 0x6C,
                        0x70, 0x61, 0x74, 0x68, 0x20, 0x54, 0x65, 0x6D, 0x70, 0x6C, 0x61, 0x74,
                        0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
