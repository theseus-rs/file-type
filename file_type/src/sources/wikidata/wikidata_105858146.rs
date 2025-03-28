use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858146: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_146,
        source_type: SourceType::Wikidata,
        name: "Maptapper settings",
        extensions: &["ini"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x46, 0x69, 0x6C, 0x65, 0x5D, 0x0D, 0x0A, 0x47, 0x66, 0x78, 0x46,
                        0x69, 0x6C, 0x65, 0x4E, 0x61, 0x6D, 0x65, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
