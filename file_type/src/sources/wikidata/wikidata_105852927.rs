use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852927: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_927,
        source_type: SourceType::Wikidata,
        name: "Creature House Expression Skeletal Stroke",
        extensions: &["sks"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x53, 0x4B, 0x33, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
