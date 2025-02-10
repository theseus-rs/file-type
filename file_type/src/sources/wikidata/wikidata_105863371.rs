use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863371: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_371,
        source_type: SourceType::Wikidata,
        name: "magic compiled data (BE)",
        extensions: &["mgc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF1, 0x1E, 0x04, 0x1C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
