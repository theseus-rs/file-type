use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863571: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_571,
        source_type: SourceType::Wikidata,
        name: "MMFW Scripts (v3)",
        extensions: &["mms"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4D, 0x46, 0x57, 0x20, 0x33, 0x20, 0x53, 0x63, 0x72, 0x69, 0x70,
                        0x74, 0x00, 0x00, 0x00, 0x4D, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
