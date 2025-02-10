use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855467: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_467,
        source_type: SourceType::Wikidata,
        name: "Faery Tale Adventure 2 save game",
        extensions: &["sav"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x54, 0x41, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
