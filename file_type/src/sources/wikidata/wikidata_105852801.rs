use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852801: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_801,
        source_type: SourceType::Wikidata,
        name: "Just Cause game data archive",
        extensions: &["sab"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x53, 0x57, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
