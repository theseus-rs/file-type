use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854471: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_471,
        source_type: SourceType::Wikidata,
        name: "AMX Mod X plugin",
        extensions: &["amxx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x58, 0x4D, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
