use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849718: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_718,
        source_type: SourceType::Wikidata,
        name: "UCDOS Configuration",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x43, 0x44, 0x4F, 0x53, 0x20, 0x43, 0x4F, 0x4E, 0x46, 0x49, 0x47,
                        0x20, 0x46, 0x49, 0x4C, 0x45, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
