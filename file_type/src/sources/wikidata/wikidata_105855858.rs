use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855858: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_858,
        source_type: SourceType::Wikidata,
        name: "Dawn file format",
        extensions: &["dwn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x41, 0x57, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
