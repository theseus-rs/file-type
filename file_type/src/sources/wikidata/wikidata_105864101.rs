use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864101: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_101,
        source_type: SourceType::Wikidata,
        name: "DeskTop Tracker module",
        extensions: &["dtt"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x73, 0x6B, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
