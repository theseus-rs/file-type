use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854860: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_860,
        source_type: SourceType::Wikidata,
        name: "Squish compressed",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0xB2, 0x90, 0xF4])],
                },
            }],
        }],
        related_formats: &[],
    },
};
