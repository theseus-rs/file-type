use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857878: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_878,
        source_type: SourceType::Wikidata,
        name: "Davex archived volume image",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x60, 0x56, 0x53, 0x54, 0x4F, 0x52, 0x45, 0x20, 0x5B, 0x44, 0x61, 0x76,
                        0x65, 0x78, 0x5D, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
