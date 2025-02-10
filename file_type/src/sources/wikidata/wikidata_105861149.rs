use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861149: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_149,
        source_type: SourceType::Wikidata,
        name: "CUPL error Listing",
        extensions: &["lst"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x49, 0x53, 0x54, 0x49, 0x4E, 0x47, 0x20, 0x46, 0x4F, 0x52, 0x20,
                        0x4C, 0x4F, 0x47, 0x49, 0x43, 0x20, 0x44, 0x45, 0x53, 0x43, 0x52, 0x49,
                        0x50, 0x54, 0x49, 0x4F, 0x4E, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
