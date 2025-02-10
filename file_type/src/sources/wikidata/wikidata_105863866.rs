use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863866: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_866,
        source_type: SourceType::Wikidata,
        name: "MK-Jamz Ad Lib module",
        extensions: &["mkj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4B, 0x4A, 0x61, 0x6D, 0x7A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
