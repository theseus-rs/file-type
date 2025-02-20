use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854937: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_937,
        source_type: SourceType::Wikidata,
        name: "AnimationWorks Accelerated movie",
        extensions: &["awa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x77, 0x61, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
