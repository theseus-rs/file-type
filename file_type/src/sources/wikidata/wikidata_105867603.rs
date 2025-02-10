use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867603: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_603,
        source_type: SourceType::Wikidata,
        name: "NetEase Games data Package",
        extensions: &["npk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x58, 0x50, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
