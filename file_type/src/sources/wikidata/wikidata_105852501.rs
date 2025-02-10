use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852501: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_501,
        source_type: SourceType::Wikidata,
        name: "Cosmigo Pro Motion SPRites sequence/animation",
        extensions: &["spr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
