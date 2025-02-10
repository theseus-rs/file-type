use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863415: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_415,
        source_type: SourceType::Wikidata,
        name: "StarTrekker 8-channel module",
        extensions: &["mod"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4C, 0x54, 0x38])],
                },
            }],
        }],
        related_formats: &[],
    },
};
